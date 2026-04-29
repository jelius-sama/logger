#define STRING_IMPLEMENTATION
#include "logger.h"
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

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

String MTTempl(const char *body, ...) {
  va_list args;
  va_start(args, body);

  // Calculate size: template length + space for appended digits
  int template_len = strlen(body);
  int extra_space = 0;
  va_list args_copy;
  va_copy(args_copy, args);

  // Count digits needed until sentinel (-1)
  int arg;
  while ((arg = va_arg(args_copy, int)) != -1) {
    extra_space += snprintf(NULL, 0, "%d", arg);
  }
  va_end(args_copy);

  int total_size = template_len + extra_space + 1;
  char *buffer = (char *)malloc(total_size);

  // Copy template
  strcpy(buffer, body);

  // Append each variadic argument as a digit
  va_start(args, body);
  char *pos = buffer + template_len;
  int arg_val;
  while ((arg_val = va_arg(args, int)) != -1) {
    pos += sprintf(pos, "%d", arg_val);
  }
  va_end(args);

  return (String){.data = buffer, .len = strlen(buffer)};
}

void FreeMTTempl(String s) { free((void *)s.data); }

void free_string(String str) { free((void *)str.data); }
