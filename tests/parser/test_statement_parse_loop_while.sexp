=== Parse simple while loop
while (i < 10) {
  i = i + 1;
}
---
(Program
  (While
    (Binary LessThan (Identifier i) (NumericLiteral 10))
    (Block
      (Expression
        (Assignment Assign
          (Identifier i)
          (Binary Add (Identifier i) (NumericLiteral 1)))))))

=== Parse simple while loop with boolean condition
while (true) {
  i = i + 1;
}
---
(Program
  (While
    (BooleanLiteral true)
    (Block
      (Expression
        (Assignment Assign
          (Identifier i)
          (Binary Add (Identifier i) (NumericLiteral 1)))))))

=== Parse while loop with complex condition
while (x > 5 && y < 20) {
  x = x - 1;
  y = y + 1;
}
---
(Program
  (While
    (Logical And
      (Binary GreaterThan (Identifier x) (NumericLiteral 5))
      (Binary LessThan (Identifier y) (NumericLiteral 20)))
    (Block
      (Expression
        (Assignment Assign
          (Identifier x)
          (Binary Subtract (Identifier x) (NumericLiteral 1))))
      (Expression
        (Assignment Assign
          (Identifier y)
          (Binary Add (Identifier y) (NumericLiteral 1)))))))

=== Parse empty while loop
while (condition) {
}
---
(Program
  (While (Identifier condition) (Block)))

=== Parse nested while loops
while (outer > 0) {
  outer = outer - 1;

  while (inner < 5) {
      inner = inner + 1;
  }
}
---
(Program
  (While
    (Binary GreaterThan (Identifier outer) (NumericLiteral 0))
    (Block
      (Expression
        (Assignment Assign
          (Identifier outer)
          (Binary Subtract (Identifier outer) (NumericLiteral 1))))
      (While
        (Binary LessThan (Identifier inner) (NumericLiteral 5))
        (Block
          (Expression
            (Assignment Assign
              (Identifier inner)
              (Binary Add (Identifier inner) (NumericLiteral 1)))))))))

=== Parse while loop with unary condition
while (!done) {
  x = x + 1;
}
---
(Program
  (While
    (Unary Not (Identifier done))
    (Block
      (Expression
        (Assignment Assign
          (Identifier x)
          (Binary Add (Identifier x) (NumericLiteral 1)))))))
