package main

/*
#include "logger.h"
*/
import "C"
import (
    "fmt"
)

func main() {
    msg := fmt.Sprintf("Addition result: %d", 668+669)

    C.Configure(C.LDebug, C.SBrackets)

    C.Debug(msg)
    C.Info(msg)
    C.Okay(msg)
    C.Warn(msg)
    C.Error(msg)
    C.Fatal(msg)
    C.Panic(msg)

    fmt.Printf("\n")
    C.Configure(C.LDebug, C.SColon)

    C.Debug(msg)
    C.Info(msg)
    C.Okay(msg)
    C.Warn(msg)
    C.Error(msg)
    C.Fatal(msg)
    C.Panic(msg)

    fmt.Printf("\n")
    C.Configure(C.LDebug, C.SNone)

    C.Debug(msg)
    C.Info(msg)
    C.Okay(msg)
    C.Warn(msg)
    C.Error(msg)
    C.Fatal(msg)
    C.Panic(msg)
}

