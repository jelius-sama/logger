package main

/*
#include "logger.h"
#include <stdlib.h>
*/
import "C"
import (
    "fmt"
    "unsafe"
)

func main() {
    result := 668 + 669
    msg := fmt.Sprintf("Addition result: %d", result)

    cMsg := C.CString(msg)
    defer C.free(unsafe.Pointer(cMsg))

    // Configure and Log
    C.Configure(C.LDebug, C.SBrackets)
    C.Debug(cMsg)
}

