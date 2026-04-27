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

// Inspired from: https://youtu.be/y8PLpDgZc0E?si=lQPnn4Nokze-aviu
#ifdef STRING_IMPLEMENTATION
#include <stdarg.h>

typedef struct {
  const char *data;
  int64_t len;
} String;

void free_string(String);
String string(const char *, ...);
void Debug(const String msg);
void Info(const String msg);
void Okay(const String msg);
void Warn(const String msg);
void Error(const String msg);
void Fatal(const String msg);
void Panic(const String msg);
#else
void Info(const _GoString_ msg);
void Debug(const _GoString_ msg);
void Okay(const _GoString_ msg);
void Warn(const _GoString_ msg);
void Error(const _GoString_ msg);
void Fatal(const _GoString_ msg);
void Panic(const _GoString_ msg);
#endif // STRING_IMPLEMENTATION

#endif // LOGGER_H
