package main

/*
#include "logger.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

void logger(void) {
    FILE *f = fopen("log.txt", "a");
    if (f == NULL) return;
    fprintf(f, "hello from go\n");
    fclose(f);
}

typedef struct {
    ActionItem *item;
    Action *action;
} AutoConfT;

AutoConfT AutoConf() {
    ActionItem *item = malloc(sizeof(ActionItem));
    item->choice = ChoiceCallback;
    item->action.callback = logger;

    Action *action = malloc(sizeof(Action));
    memset(action, 0, sizeof(Action));
    action->on_error = item;

    Configure(LDebug, SBrackets, action);
    return (AutoConfT){
        .action = action,
        .item = item
    };
}

void FreeAutoConf(AutoConfT a) {
    free(a.action);
    free(a.item);
}
*/
import "C"
import (
    "fmt"
)

func main() {
    msg := fmt.Sprintf("Addition result: %d", 668+669)

    a := C.AutoConf()

    C.Debug(msg)
    C.Info(msg)
    C.Okay(msg)
    C.Warn(msg)
    C.Error(msg)
    C.Fatal(msg)
    C.Panic(msg)

    C.FreeAutoConf(a)
}

