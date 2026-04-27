#ifndef LOGGER_H
#define LOGGER_H

#include <stddef.h>
#include <stdint.h>

typedef enum {
  LDebug = 0,
  LInfo = 1,
  LOkay = 2,
  LWarn = 3,
  LError = 4,
  LFatal = 5,
  LPanic = 6,
} LogLevel;

typedef enum {
  SBrackets = 0,
  SColon = 1,
  SNone = 2,
} LogStyle;

void Configure(LogLevel level, LogStyle style);

#ifdef STRING_IMPLEMENTATION
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
  const char *data;
  int64_t len;
} String;

int vasprintf(char **strp, const char *fmt, va_list ap) {
  va_list ap_copy;
  va_copy(ap_copy, ap);

  int len = vsnprintf(NULL, 0, fmt, ap_copy);
  va_end(ap_copy);

  if (len < 0)
    return -1;

  *strp = (char *)malloc(len + 1);
  if (!*strp)
    return -1;

  return vsnprintf(*strp, len + 1, fmt, ap);
}

String string(const char *format, ...) {
  char *ptr = NULL;
  va_list args;

  va_start(args, format);
  // vasprintf automatically allocates memory for the formatted string
  int len = vasprintf(&ptr, format, args);
  va_end(args);

  if (len < 0) {
    return (String){.data = "", .len = 0};
  }

  return (String){.data = ptr, .len = (int64_t)len};
}

void Info(const String msg);
void Debug(const String msg);
#else
void Info(const _GoString_ msg);
void Debug(const _GoString_ msg);
#endif // STRING_IMPLEMENTATION

#endif // LOGGER_H
