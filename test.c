#include <stdio.h>
#define STRING_IMPLEMENTATION
#include "logger.h"

int main(void) {
  String msg = string("Addition result: %d", 210 + 210);

  Configure(LDebug, SBrackets);

  Debug(msg);
  Info(msg);
  Okay(msg);
  Warn(msg);
  Error(msg);
  Fatal(msg);
  Panic(msg);

  printf("\n");
  Configure(LDebug, SColon);

  Debug(msg);
  Info(msg);
  Okay(msg);
  Warn(msg);
  Error(msg);
  Fatal(msg);
  Panic(msg);

  printf("\n");
  Configure(LDebug, SNone);

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
