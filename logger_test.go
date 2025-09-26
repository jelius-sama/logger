package logger

import (
	"bytes"
	"os"
	"strings"
	"testing"
)

func TestSetStyle(t *testing.T) {
	// Capture stdout to suppress output during tests
	oldStdout := os.Stdout
	_, w, _ := os.Pipe()
	os.Stdout = w

	defer func() {
		w.Close()
		os.Stdout = oldStdout
	}()

	// Test valid styles
	SetStyle("brackets")
	if LoggerStyle != "brackets" {
		t.Errorf("Expected LoggerStyle to be 'brackets', got '%s'", LoggerStyle)
	}

	SetStyle("colon")
	if LoggerStyle != "colon" {
		t.Errorf("Expected LoggerStyle to be 'colon', got '%s'", LoggerStyle)
	}

	// Test invalid style (should default to brackets)
	SetStyle("invalid")
	if LoggerStyle != "brackets" {
		t.Errorf("Expected LoggerStyle to default to 'brackets', got '%s'", LoggerStyle)
	}
}

func TestApplyStyle(t *testing.T) {
	// Test brackets style
	LoggerStyle = "brackets"
	result := applyStyle("%s Test", "INFO")
	expected := "[INFO] Test"
	if result != expected {
		t.Errorf("Expected '%s', got '%s'", expected, result)
	}

	// Test colon style
	LoggerStyle = "colon"
	result = applyStyle("%s Test", "ERROR")
	expected = "ERROR: Test"
	if result != expected {
		t.Errorf("Expected '%s', got '%s'", expected, result)
	}
}

func TestLoggerFunctions(t *testing.T) {
	// Capture stdout and stderr
	oldStdout := os.Stdout
	oldStderr := os.Stderr

	rOut, wOut, _ := os.Pipe()
	rErr, wErr, _ := os.Pipe()

	os.Stdout = wOut
	os.Stderr = wErr

	// Test all non-fatal/non-panic logging functions
	tests := []struct {
		name     string
		function func(...any)
		isStderr bool
	}{
		{"Info", Info, false},
		{"Debug", Debug, false},
		{"Okay", Okay, false},
		{"Warning", Warning, false},
		{"Error", Error, true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			tt.function("Test message")
			// Just verify the function doesn't crash
		})
	}

	// Close writers and restore
	wOut.Close()
	wErr.Close()
	os.Stdout = oldStdout
	os.Stderr = oldStderr

	// Read and verify some output was produced
	bufOut := make([]byte, 1024)
	bufErr := make([]byte, 1024)

	rOut.Read(bufOut)
	rErr.Read(bufErr)

	// Verify that some output was produced
	if len(bytes.Trim(bufOut, "\x00")) == 0 && len(bytes.Trim(bufErr, "\x00")) == 0 {
		t.Error("Expected some output from logging functions")
	}
}

func TestTimedLoggerFunctions(t *testing.T) {
	// Capture stdout and stderr
	oldStdout := os.Stdout
	oldStderr := os.Stderr

	rOut, wOut, _ := os.Pipe()
	rErr, wErr, _ := os.Pipe()

	os.Stdout = wOut
	os.Stderr = wErr

	// Test timed logging functions
	TimedInfo("Timed info test")
	TimedDebug("Timed debug test")
	TimedOkay("Timed okay test")
	TimedWarning("Timed warning test")
	TimedError("Timed error test")

	wOut.Close()
	wErr.Close()
	os.Stdout = oldStdout
	os.Stderr = oldStderr

	// Read output
	bufOut := make([]byte, 2048)
	bufErr := make([]byte, 2048)

	nOut, _ := rOut.Read(bufOut)
	nErr, _ := rErr.Read(bufErr)

	output := string(bufOut[:nOut]) + string(bufErr[:nErr])

	// Verify timestamp format is present (YYYY/MM/DD HH:MM:SS)
	if !strings.Contains(output, "/") || !strings.Contains(output, ":") {
		t.Error("Expected timed logging to include timestamp")
	}
}

func TestPanicRecovery(t *testing.T) {
	// Capture stderr
	oldStderr := os.Stderr
	rErr, wErr, _ := os.Pipe()
	os.Stderr = wErr

	defer func() {
		wErr.Close()
		os.Stderr = oldStderr

		// Verify panic occurred
		if r := recover(); r == nil {
			t.Error("Expected Panic function to panic")
		} else {
			// Verify panic message
			panicMsg := r.(string)
			if !strings.Contains(panicMsg, "Test panic message") {
				t.Errorf("Expected panic message to contain 'Test panic message', got '%s'", panicMsg)
			}
		}

		// Read stderr output
		buf := make([]byte, 1024)
		n, _ := rErr.Read(buf)
		output := string(buf[:n])

		// Verify PANIC label was printed
		if !strings.Contains(output, "PANIC") {
			t.Error("Expected PANIC label in stderr output")
		}
	}()

	// This should panic
	Panic("Test panic message")
}

func TestFatalFunction(t *testing.T) {
	// We can't easily test Fatal since it calls os.Exit
	// But we can test that it exists and has the right signature
	t.Log("Fatal function exists but cannot be tested due to os.Exit call")
}

// Benchmark tests
func BenchmarkInfo(b *testing.B) {
	// Redirect output to discard
	oldStdout := os.Stdout
	os.Stdout, _ = os.Open(os.DevNull)
	defer func() { os.Stdout = oldStdout }()

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		Info("Benchmark test message")
	}
}

func BenchmarkError(b *testing.B) {
	// Redirect output to discard
	oldStderr := os.Stderr
	os.Stderr, _ = os.Open(os.DevNull)
	defer func() { os.Stderr = oldStderr }()

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		Error("Benchmark test message")
	}
}

// Example tests that show up in documentation
func ExampleInfo() {
	Info("This is an info message")
	// Output will include colored [INFO] prefix
}

func ExampleSetStyle() {
	SetStyle("colon")
	Info("This message uses colon style")

	SetStyle("brackets")
	Info("This message uses bracket style")
}

func ExampleTimedInfo() {
	TimedInfo("This message includes a timestamp")
	// Output will include timestamp like: 2006/01/02 15:04:05 This message includes a timestamp
}
