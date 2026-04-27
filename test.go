package main

/*
#include "logger.h"
*/
import "C"
import (
    "fmt"
)

func main() {
    result := 668 + 669

    // Configure and Log
    C.Configure(C.LDebug, C.SBrackets)

    C.Debug(fmt.Sprintf("Addition result: %d", result))
    C.Info(fmt.Sprintf("Addition result: %d", result))
}

