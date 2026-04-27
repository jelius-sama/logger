#ifndef LOGGER_H
#define LOGGER_H

#include <stdint.h>

typedef enum { LDebug = 0, LInfo = 1, LError = 2 } LogLevel;

typedef enum {
  SBrackets = 0,
  SColon = 1,
  SNone = 2,
} LogStyle;

void Configure(LogLevel level, LogStyle style);
void Debug(char *msg);

typedef struct {
  const char *data;
  int64_t len;
} String;

#endif // LOGGER_H
