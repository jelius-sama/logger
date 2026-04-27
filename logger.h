typedef enum { LDebug = 0, LInfo = 1, LError = 2 } LogLevel;

typedef enum {
  SBrackets = 0,
  SColon = 1,
  SNone = 2,
} LogStyle;

void Configure(LogLevel level, LogStyle style);
void Debug(const char *msg);
