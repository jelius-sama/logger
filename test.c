#include <stdio.h>
#define STRING_IMPLEMENTATION
#include "logger.h"
#include <unistd.h>

void logger(void) {
  FILE *f = fopen("log.txt", "a");
  if (f == NULL)
    return;
  fprintf(f, "hello\n");
  fclose(f);
}

int main(void) {
  String msg = string("Addition result: %d", 210 + 210);
  Action action = {
      .on_error = &(ActionItem){.choice = ChoiceCallback, .action = logger}};

  Configure(LDebug, SBrackets, &action);

  Debug(msg);
  Info(msg);
  Okay(msg);
  Warn(msg);
  Error(msg);
  sleep(2);
  Fatal(msg);
  Panic(msg);

  printf("\n");
  Configure(LDebug, SColon, &action);

  Debug(msg);
  Info(msg);
  Okay(msg);
  Warn(msg);
  Error(msg);
  Fatal(msg);
  Panic(msg);

  printf("\n");
  Configure(LDebug, SNone, &action);

  Debug(msg);
  Info(msg);
  Okay(msg);
  Warn(msg);
  Error(msg);
  Fatal(msg);
  Panic(msg);

  free_string(msg);

  return 0;
}
