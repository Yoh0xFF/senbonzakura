==============================================
Parse binary expression with additive operator
==============================================
2 + 5;
---
(Program
  (Expression (Binary Add (NumericLiteral 2) (NumericLiteral 5))))

============================================
Parse binary expression with factor operator
============================================
2 * 5;
---
(Program
  (Expression (Binary Multiply (NumericLiteral 2) (NumericLiteral 5))))

========================================================
Parse binary expression with multiple additive operators
========================================================
1 + 8 - 2;
---
(Program
  (Expression
    (Binary Subtract
      (Binary Add (NumericLiteral 1) (NumericLiteral 8))
      (NumericLiteral 2))))

======================================================
Parse binary expression with multiple factor operators
======================================================
2 * 9 / 3;
---
(Program
  (Expression
    (Binary Divide
      (Binary Multiply (NumericLiteral 2) (NumericLiteral 9))
      (NumericLiteral 3))))

========================================
Parse binary expression with parentheses
========================================
1 + (8 - 2);
---
(Program
  (Expression
    (Binary Add
      (NumericLiteral 1)
      (Binary Subtract (NumericLiteral 8) (NumericLiteral 2)))))

==========================================================
Parse binary expression with additive and factor operators
==========================================================
1 + 4 * 5;
---
(Program
  (Expression
    (Binary Add
      (NumericLiteral 1)
      (Binary Multiply (NumericLiteral 4) (NumericLiteral 5)))))

=======================================================================
Parse binary expression with additive, factor operators and parentheses
=======================================================================
(1 + 4) * 5;
---
(Program
  (Expression
    (Binary Multiply
      (Binary Add (NumericLiteral 1) (NumericLiteral 4))
      (NumericLiteral 5))))
