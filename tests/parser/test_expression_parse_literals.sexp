==========================
Parse boolean true literal
==========================
true;
---
(Program
  (Expression (BooleanLiteral true)))

===========================
Parse boolean false literal
===========================
false;
---
(Program
  (Expression (BooleanLiteral false)))

=================
Parse nil literal
=================
nil;
---
(Program
  (Expression (NilLiteral)))

=================================
Parse double quote string literal
=================================
"Hello";
---
(Program
  (Expression (StringLiteral "Hello")))

=================================
Parse single quote string literal
=================================
'Hello';
---
(Program
  (Expression (StringLiteral "Hello")))

=============================
Parse multiple string literal
=============================
'Hello'; "World";
---
(Program
  (Expression (StringLiteral "Hello"))
  (Expression (StringLiteral "World")))

=====================
Parse numeric literal
=====================
17;
---
(Program
  (Expression (NumericLiteral 17)))

=====================
Parsed mixed literals
=====================
'Hello';
"World";
17;
---
(Program
  (Expression (StringLiteral "Hello"))
  (Expression (StringLiteral "World"))
  (Expression (NumericLiteral 17)))
