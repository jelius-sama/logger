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

void free_string(String str) { free((void *)str.data); }
