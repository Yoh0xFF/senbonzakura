=========================
Parse unary plus operator
=========================
+42;
---
(Program
  (Expression (Unary Plus (NumericLiteral 42))))

==========================
Parse unary minus operator
==========================
-42;
---
(Program
  (Expression (Unary Minus (NumericLiteral 42))))

========================
Parse unary not operator
========================
!true;
---
(Program
  (Expression (Unary Not (BooleanLiteral true))))

===============================
Parse unary not with identifier
===============================
!isValid;
---
(Program
  (Expression (Unary Not (Identifier isValid))))

=============================
Parse chained unary operators
=============================
+-42;
---
(Program
  (Expression (Unary Plus (Unary Minus (NumericLiteral 42)))))

======================================
Parse multiple chained unary operators
======================================
+-!x;
---
(Program
  (Expression (Unary Plus (Unary Minus (Unary Not (Identifier x))))))

=====================================
Parse unary operator with parentheses
=====================================
-(x + y);
---
(Program
  (Expression (Unary Minus (Binary Add (Identifier x) (Identifier y)))))

======================================================
Parse unary operators precedence with binary operators
======================================================
-x + y;
---
(Program
  (Expression (Binary Add (Unary Minus (Identifier x)) (Identifier y))))

=====================================
Parse unary operators with assignment
=====================================
result = -x;
---
(Program
  (Expression
    (Assignment Assign (Identifier result) (Unary Minus (Identifier x)))))

==============================================================
Parse unary minus with numeric literal in variable declaration
==============================================================
let x: number = -5;
---
(Program
  (VariableDeclaration
    (Variable (Identifier x) Number (Unary Minus (NumericLiteral 5)))))

===========================================
Parse unary operators in binary expressions
===========================================
x * -y;
---
(Program
  (Expression (Binary Multiply (Identifier x) (Unary Minus (Identifier y)))))

=========================================
Parse unary not with equality expressions
=========================================
!(x == y);
---
(Program
  (Expression (Unary Not (Binary Equal (Identifier x) (Identifier y)))))

===============================================
Parse unary operators with multiple expressions
===============================================
-x + -y;
---
(Program
  (Expression
    (Binary Add (Unary Minus (Identifier x)) (Unary Minus (Identifier y)))))

================================================
Parse unary operators in conditional expressions
================================================
if (!condition) {
    x = -y;
}
---
(Program
  (If
    (Unary Not (Identifier condition))
    (Block
      (Expression
        (Assignment Assign (Identifier x) (Unary Minus (Identifier y)))))))

========================================
Parse unary not with logical expressions
========================================
!(x && y);
---
(Program
  (Expression (Unary Not (Logical And (Identifier x) (Identifier y)))))

====================================================
Parse unary operators precedence with multiplication
====================================================
-x * y;
---
(Program
  (Expression (Binary Multiply (Unary Minus (Identifier x)) (Identifier y))))

=========================================
Parse unary not with variable declaration
=========================================
let isNotValid: boolean = !isValid;
---
(Program
  (VariableDeclaration
    (Variable (Identifier isNotValid) Boolean (Unary Not (Identifier isValid)))))

=============================================
Parse complex expression with unary operators
=============================================
result = -x * y + !z && w;
---
(Program
  (Expression
    (Assignment Assign
      (Identifier result)
      (Logical And
        (Binary Add
          (Binary Multiply (Unary Minus (Identifier x)) (Identifier y))
          (Unary Not (Identifier z)))
        (Identifier w)))))
