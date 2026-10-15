=======================
Parse simple assignment
=======================
x = 5;
---
(Program
  (Expression (Assignment Assign (Identifier x) (NumericLiteral 5))))

========================
Parse complex assignment
========================
x += 10;
---
(Program
  (Expression (Assignment AssignAdd (Identifier x) (NumericLiteral 10))))

=======================================
Parse assignment with binary expression
=======================================
x = 5 + 12;
---
(Program
  (Expression
    (Assignment Assign
      (Identifier x)
      (Binary Add (NumericLiteral 5) (NumericLiteral 12)))))

========================
Parse chained assignment
========================
x = y = 5;
---
(Program
  (Expression
    (Assignment Assign
      (Identifier x)
      (Assignment Assign (Identifier y) (NumericLiteral 5)))))
