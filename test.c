#include "logger.h"
#include <stdio.h>

int main(void) {
  // Perform addition in C
  int result = 210 + 210;
  char msg[50];
  sprintf(msg, "Addition result: %d", result);

  // Setup the global logger state
  Configure(LDebug, SBrackets);

  // Call the Rust library
  Debug(msg);

  return 0;
}
