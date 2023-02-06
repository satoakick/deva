
delim [ \t]+
ws    {delim}+

letter [A-Za-z]
digit  [0-9]

ident  {letter}({letter}|{digit})*

number {digit}+(\.{digit}+)?(E[+\-]?{digit}+)?

%%

{ws}      { putchar(' '); }
{number}  { printf("<b>"); ECHO; printf("</b>"); }
{ident}   { printf("<i>"); ECHO; printf("</i>"); }
"."       { ECHO; exit(1); }
.|\n      { ECHO; }
%%

int main(void) {
  return yylex();
}
