#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define STRING_IMPLEMENTATION
#include "logger.h"

void logger(void) {
  FILE *f = fopen("log.txt", "a");
  if (f == NULL)
    return;
  printf("callback called\n");
  fprintf(f, "hello\n");
  fclose(f);
}

int main(void) {
  String msg = string("Addition result: %d", 210 + 210);

  char **cc_array = malloc(2 * sizeof(char *));
  cc_array[0] = strdup("jelius.basumatary.sama@gmail.com");
  cc_array[1] = strdup("work@jelius.dev");

  StrArr *cc = malloc(sizeof(StrArr));
  cc->str = (char *)cc_array;
  cc->count = 2;
  cc->len = 2 * sizeof(char *);

  DefaultMailAction mail = {
      .cfg_path = NULL,
      .body_template =
          MTTempl("An error occurred.\n\tLevel: %s\n\t%s logs: %s\n", MTLLevel,
                  MTLLevel, MTMessage, -1),
      .title = string("Error Alert"),
      .to = string("personal@jelius.dev"),
      .cc = cc,
      .bcc = NULL,
  };

  Action action = {.on_error = &(ActionItem){.choice = ChoiceMail,
                                             .action = {.send_mail = mail}}};
  Configure(LDebug, SBrackets, &action);

  Debug(msg);
  Info(msg);
  Okay(msg);
  Warn(msg);
  Error(msg);
  Fatal(msg);
  Panic(msg);

  printf("\n");
  Configure(LDebug, SColon, &action);

  Debug(msg);
  Info(msg);
  Okay(msg);
  Warn(msg);
  Error(msg);
  Fatal(msg);
  Panic(msg);

  printf("\n");
  Configure(LDebug, SNone, &action);

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
