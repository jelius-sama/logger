package main

/*
#include "logger.h"
#include <stdlib.h>
*/
import "C"
import (
    "fmt"
)

func main() {
    result := 668 + 669

    // Configure and Log
    C.Configure(C.LDebug, C.SBrackets)
    C.Debug(C.CString(fmt.Sprintf("Addition result: %d", result)))

    C.Info(fmt.Sprintf("Addition result: %d", result))
}

