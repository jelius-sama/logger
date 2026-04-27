#define STRING_IMPLEMENTATION
#include "logger.h"

int main(void) {
  String msg = string("Addition result: %d", 210 + 210);

  // Setup the global logger state
  Configure(LDebug, SBrackets);

  Debug(msg);
  Info(msg);

  return 0;
}
