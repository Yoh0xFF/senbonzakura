=====================
Parse simple for loop
=====================
for (let i: number = 0; i < 10; i = i + 1) {
  sum = sum + i;
}
---
(Program
  (For
    (VariableDeclaration (Variable (Identifier i) Number (NumericLiteral 0)))
    (Binary LessThan (Identifier i) (NumericLiteral 10))
    (Assignment Assign
      (Identifier i)
      (Binary Add (Identifier i) (NumericLiteral 1)))
    (Block
      (Expression
        (Assignment Assign
          (Identifier sum)
          (Binary Add (Identifier sum) (Identifier i)))))))

==========================================
Parse for loop with initializer expression
==========================================
for (i = 0; i < 10; i = i + 1) {
  sum = sum + i;
}
---
(Program
  (For
    (Expression (Assignment Assign (Identifier i) (NumericLiteral 0)))
    (Binary LessThan (Identifier i) (NumericLiteral 10))
    (Assignment Assign
      (Identifier i)
      (Binary Add (Identifier i) (NumericLiteral 1)))
    (Block
      (Expression
        (Assignment Assign
          (Identifier sum)
          (Binary Add (Identifier sum) (Identifier i)))))))

============================================
Parse for loop with multiple initializations
============================================
for (let i: number = 0, j: number = 10; i < j; i = i + 1) {
  sum = sum + i;
}
---
(Program
  (For
    (VariableDeclaration
      (Variable (Identifier i) Number (NumericLiteral 0))
      (Variable (Identifier j) Number (NumericLiteral 10)))
    (Binary LessThan (Identifier i) (Identifier j))
    (Assignment Assign
      (Identifier i)
      (Binary Add (Identifier i) (NumericLiteral 1)))
    (Block
      (Expression
        (Assignment Assign
          (Identifier sum)
          (Binary Add (Identifier sum) (Identifier i)))))))

=====================================
Parse for loop without initialization
=====================================
for (; i < 10; i = i + 1) {
  sum = sum + i;
}
---
(Program
  (For
    _
    (Binary LessThan (Identifier i) (NumericLiteral 10))
    (Assignment Assign
      (Identifier i)
      (Binary Add (Identifier i) (NumericLiteral 1)))
    (Block
      (Expression
        (Assignment Assign
          (Identifier sum)
          (Binary Add (Identifier sum) (Identifier i)))))))

================================
Parse for loop without condition
================================
for (let i: number = 0; ; i = i + 1) {
  if (i >= 10) {
    i = 0;
  }
  sum = sum + i;
}
---
(Program
  (For
    (VariableDeclaration (Variable (Identifier i) Number (NumericLiteral 0)))
    _
    (Assignment Assign
      (Identifier i)
      (Binary Add (Identifier i) (NumericLiteral 1)))
    (Block
      (If
        (Binary GreaterThanOrEqualTo (Identifier i) (NumericLiteral 10))
        (Block
          (Expression (Assignment Assign (Identifier i) (NumericLiteral 0)))))
      (Expression
        (Assignment Assign
          (Identifier sum)
          (Binary Add (Identifier sum) (Identifier i)))))))

================================
Parse for loop without increment
================================
for (let i: number = 0; i < 10;) {
  sum = sum + i;
  i = i + 1;
}
---
(Program
  (For
    (VariableDeclaration (Variable (Identifier i) Number (NumericLiteral 0)))
    (Binary LessThan (Identifier i) (NumericLiteral 10))
    _
    (Block
      (Expression
        (Assignment Assign
          (Identifier sum)
          (Binary Add (Identifier sum) (Identifier i))))
      (Expression
        (Assignment Assign
          (Identifier i)
          (Binary Add (Identifier i) (NumericLiteral 1)))))))

====================
Parse empty for loop
====================
for (;;) {
  if (condition) {
    done = true;
  }
}
---
(Program
  (For
    _
    _
    _
    (Block
      (If
        (Identifier condition)
        (Block
          (Expression
            (Assignment Assign (Identifier done) (BooleanLiteral true))))))))

======================
Parse nested for loops
======================
for (let i: number = 0; i < 3; i = i + 1) {
  for (let j: number = 0; j < 3; j = j + 1) {
    result = i * j;
  }
}
---
(Program
  (For
    (VariableDeclaration (Variable (Identifier i) Number (NumericLiteral 0)))
    (Binary LessThan (Identifier i) (NumericLiteral 3))
    (Assignment Assign
      (Identifier i)
      (Binary Add (Identifier i) (NumericLiteral 1)))
    (Block
      (For
        (VariableDeclaration
          (Variable (Identifier j) Number (NumericLiteral 0)))
        (Binary LessThan (Identifier j) (NumericLiteral 3))
        (Assignment Assign
          (Identifier j)
          (Binary Add (Identifier j) (NumericLiteral 1)))
        (Block
          (Expression
            (Assignment Assign
              (Identifier result)
              (Binary Multiply (Identifier i) (Identifier j)))))))))

======================================
Parse for loop with complex expression
======================================
for (let i: number = a * b + c; i < max && !done; i = i + 1) {
  sum = sum + i;
}
---
(Program
  (For
    (VariableDeclaration
      (Variable
        (Identifier i)
        Number
        (Binary Add
          (Binary Multiply (Identifier a) (Identifier b))
          (Identifier c))))
    (Logical And
      (Binary LessThan (Identifier i) (Identifier max))
      (Unary Not (Identifier done)))
    (Assignment Assign
      (Identifier i)
      (Binary Add (Identifier i) (NumericLiteral 1)))
    (Block
      (Expression
        (Assignment Assign
          (Identifier sum)
          (Binary Add (Identifier sum) (Identifier i)))))))
