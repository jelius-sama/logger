#ifndef LOGGER_H
#define LOGGER_H

#include <stddef.h>
#include <stdint.h>

typedef enum {
  LDebug = 0,
  LOkay = 1,
  LInfo = 2,
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

typedef enum {
  ChoiceMail = 0,
  ChoiceCallback = 1,
} Choice;

// Inspired from: https://youtu.be/y8PLpDgZc0E?si=lQPnn4Nokze-aviu
#ifdef STRING_IMPLEMENTATION
#include <stdarg.h>

typedef struct {
  const char *data;
  int64_t len;
} String;

typedef struct {
  String body;
  String title;
  String **to;
  String **cc;
  String **bcc;
} DefaultMailAction;

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
typedef struct {
  _GoString_ body;
  _GoString_ title;
  _GoString_ **to;
  _GoString_ **cc;
  _GoString_ **bcc;
} DefaultMailAction;

void Info(const _GoString_ msg);
void Debug(const _GoString_ msg);
void Okay(const _GoString_ msg);
void Warn(const _GoString_ msg);
void Error(const _GoString_ msg);
void Fatal(const _GoString_ msg);
void Panic(const _GoString_ msg);
#endif // STRING_IMPLEMENTATION

typedef union {
  void (*callback)(void);
  DefaultMailAction send_mail;
} ActionChoice;

typedef struct {
  Choice choice;
  ActionChoice action;
} ActionItem;

typedef struct {
  ActionItem *on_debug;
  ActionItem *on_okay;
  ActionItem *on_info;
  ActionItem *on_warn;
  ActionItem *on_error;
  ActionItem *on_panic;
  ActionItem *on_fatal;
} Action;

void Configure(LogLevel level, LogStyle style, Action *action);

#endif // LOGGER_H
