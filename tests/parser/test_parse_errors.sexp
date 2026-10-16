=== Missing right-hand side of an assignment
x = ;
---
(Error ParserError "Unexpected token: StatementEnd, expected an expression")

=== Missing variable initializer
let x: number = ;
---
(Error ParserError "Unexpected token: StatementEnd, expected an expression")

=== Missing right operand of a binary expression
1 + ;
---
(Error ParserError "Unexpected token: StatementEnd, expected an expression")

=== Missing argument after a trailing comma
foo(1, );
---
(Error ParserError "Unexpected token: ClosingParenthesis, expected an expression")

=== Missing computed member property
a[];
---
(Error ParserError "Unexpected token: ClosingBracket, expected an expression")

=== Stray closing parenthesis after empty statements
;;)
---
(Error ParserError "Unexpected token: ClosingParenthesis, expected an expression")

=== Unknown escape sequence in a string literal
x = "\q";
---
(Error LexicalError "Could not parse token: InvalidEscape { line: 1, column: 6 }")

=== Hex escape sequence with a single digit
x = "\x4";
---
(Error LexicalError "Could not parse token: InvalidEscape { line: 1, column: 6 }")

=== Hex escape sequence outside of ASCII
x = "\x80";
---
(Error LexicalError "Could not parse token: InvalidEscape { line: 1, column: 6 }")
