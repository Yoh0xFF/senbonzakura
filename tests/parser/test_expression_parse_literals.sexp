=== Parse boolean true literal
true;
---
(Program
  (Expression (BooleanLiteral true)))

=== Parse boolean false literal
false;
---
(Program
  (Expression (BooleanLiteral false)))

=== Parse nil literal
nil;
---
(Program
  (Expression (NilLiteral)))

=== Parse double quote string literal
"Hello";
---
(Program
  (Expression (StringLiteral "Hello")))

=== Parse single quote string literal
'Hello';
---
(Program
  (Expression (StringLiteral "Hello")))

=== Parse multiple string literal
'Hello'; "World";
---
(Program
  (Expression (StringLiteral "Hello"))
  (Expression (StringLiteral "World")))

=== Parse numeric literal
17;
---
(Program
  (Expression (NumericLiteral 17)))

=== Parse float numeric literal
1.5;
---
(Program
  (Expression (NumericLiteral 1.5)))

=== Parse numeric literal with exponent
1e10; 2.5E-3;
---
(Program
  (Expression (NumericLiteral 10000000000))
  (Expression (NumericLiteral 0.0025)))

=== Parse numeric literal larger than 32 bits
99999999999;
---
(Program
  (Expression (NumericLiteral 99999999999)))

=== Parsed mixed literals
'Hello';
"World";
17;
---
(Program
  (Expression (StringLiteral "Hello"))
  (Expression (StringLiteral "World"))
  (Expression (NumericLiteral 17)))
