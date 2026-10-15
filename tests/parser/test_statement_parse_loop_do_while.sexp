=== Parse simple do while loop
do {
  i = i + 1;
} while (i < 10);
---
(Program
  (DoWhile
    (Block
      (Expression
        (Assignment Assign
          (Identifier i)
          (Binary Add (Identifier i) (NumericLiteral 1)))))
    (Binary LessThan (Identifier i) (NumericLiteral 10))))

=== Parse do while loop with boolean condition
do {
  x = x + 1;
} while (true);
---
(Program
  (DoWhile
    (Block
      (Expression
        (Assignment Assign
          (Identifier x)
          (Binary Add (Identifier x) (NumericLiteral 1)))))
    (BooleanLiteral true)))

=== Parse do while loop with complex condition
do {
  x = x - 1;
  y = y + 1;
} while (x > 5 && y < 20);
---
(Program
  (DoWhile
    (Block
      (Expression
        (Assignment Assign
          (Identifier x)
          (Binary Subtract (Identifier x) (NumericLiteral 1))))
      (Expression
        (Assignment Assign
          (Identifier y)
          (Binary Add (Identifier y) (NumericLiteral 1)))))
    (Logical And
      (Binary GreaterThan (Identifier x) (NumericLiteral 5))
      (Binary LessThan (Identifier y) (NumericLiteral 20)))))

=== Parse do while loop with empty body
do {
} while (condition);
---
(Program
  (DoWhile (Block) (Identifier condition)))

=== Parse nested do while loops
do {
  outer = outer - 1;
  do {
      inner = inner + 1;
  } while (inner < 3);
} while (outer > 0);
---
(Program
  (DoWhile
    (Block
      (Expression
        (Assignment Assign
          (Identifier outer)
          (Binary Subtract (Identifier outer) (NumericLiteral 1))))
      (DoWhile
        (Block
          (Expression
            (Assignment Assign
              (Identifier inner)
              (Binary Add (Identifier inner) (NumericLiteral 1)))))
        (Binary LessThan (Identifier inner) (NumericLiteral 3))))
    (Binary GreaterThan (Identifier outer) (NumericLiteral 0))))

=== Parse do while loop with unary condition
do {
  x = x + 1;
} while (!done);
---
(Program
  (DoWhile
    (Block
      (Expression
        (Assignment Assign
          (Identifier x)
          (Binary Add (Identifier x) (NumericLiteral 1)))))
    (Unary Not (Identifier done))))
