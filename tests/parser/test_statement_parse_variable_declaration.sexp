==============================================
Parse variable declaration with initialization
==============================================
let x: number = 42;
---
(Program
  (VariableDeclaration (Variable (Identifier x) Number (NumericLiteral 42))))

=================================================
Parse variable declaration without initialization
=================================================
let x: string;
---
(Program
  (VariableDeclaration (Variable (Identifier x) String)))

==========================================================
Parse multiple variable declaration without initialization
==========================================================
let x: number = 5, y: number = 10;
---
(Program
  (VariableDeclaration
    (Variable (Identifier x) Number (NumericLiteral 5))
    (Variable (Identifier y) Number (NumericLiteral 10))))

=================================
Parse mixed variable declarations
=================================
let x: number = 5, y: number;
---
(Program
  (VariableDeclaration
    (Variable (Identifier x) Number (NumericLiteral 5))
    (Variable (Identifier y) Number)))

================================================================================
Parse multiple variable declaration with different types and with initialization
================================================================================
let count: number = 42, name: string = "John";
---
(Program
  (VariableDeclaration
    (Variable (Identifier count) Number (NumericLiteral 42))
    (Variable (Identifier name) String (StringLiteral "John"))))

=============================================
Parse variable initialization with expression
=============================================
let x: number = (2 + 3) * 4;
---
(Program
  (VariableDeclaration
    (Variable
      (Identifier x)
      Number
      (Binary Multiply
        (Binary Add (NumericLiteral 2) (NumericLiteral 3))
        (NumericLiteral 4)))))

======================================================
Parse multiple variable initialization with expression
======================================================
let x: number = 2 + 3, y: number = x * 4;
---
(Program
  (VariableDeclaration
    (Variable
      (Identifier x)
      Number
      (Binary Add (NumericLiteral 2) (NumericLiteral 3)))
    (Variable
      (Identifier y)
      Number
      (Binary Multiply (Identifier x) (NumericLiteral 4)))))
