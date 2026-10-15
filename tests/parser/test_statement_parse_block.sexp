=== Parse test block statement with string and number expressions
{
  "Hello";
  17;
}
---
(Program
  (Block
    (Expression (StringLiteral "Hello"))
    (Expression (NumericLiteral 17))))

=== Parse test empty block statement
{
}
---
(Program
  (Block))

=== Parse test nested block statement
{
  "Hello";
  {
    17;
  }
}
---
(Program
  (Block
    (Expression (StringLiteral "Hello"))
    (Block
      (Expression (NumericLiteral 17)))))

=== Parse test empty statement in block
{
  ;
}
---
(Program
  (Block
    (Empty)))
