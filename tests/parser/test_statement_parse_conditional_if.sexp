===================================================
Parse simple if statement with relational condition
===================================================
if (x > 5) {
  y = 10;
}
---
(Program
  (If
    (Binary GreaterThan (Identifier x) (NumericLiteral 5))
    (Block
      (Expression (Assignment Assign (Identifier y) (NumericLiteral 10))))))

=================================================
Parse if-else statement with relational condition
=================================================
if (x > 5) {
  y = 10;
} else {
  y = 0;
}
---
(Program
  (If
    (Binary GreaterThan (Identifier x) (NumericLiteral 5))
    (Block
      (Expression (Assignment Assign (Identifier y) (NumericLiteral 10))))
    (Block
      (Expression (Assignment Assign (Identifier y) (NumericLiteral 0))))))

=======================================================
Parse if statement with greater-than-or-equal condition
=======================================================
if (x >= 10) {
  y = 20;
}
---
(Program
  (If
    (Binary GreaterThanOrEqualTo (Identifier x) (NumericLiteral 10))
    (Block
      (Expression (Assignment Assign (Identifier y) (NumericLiteral 20))))))

=================================================
Parse if statement with boolean literal condition
=================================================
if (true) {
  x = 10;
}
---
(Program
  (If
    (BooleanLiteral true)
    (Block
      (Expression (Assignment Assign (Identifier x) (NumericLiteral 10))))))
