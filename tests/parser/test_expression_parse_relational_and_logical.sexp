=== Parse equality operator equal
x == y;
---
(Program
  (Expression (Binary Equal (Identifier x) (Identifier y))))

=== Parse equality operator not equal
x != y;
---
(Program
  (Expression (Binary NotEqual (Identifier x) (Identifier y))))

=== Parse equality operator with literals
42 == 42;
---
(Program
  (Expression (Binary Equal (NumericLiteral 42) (NumericLiteral 42))))

=== Parse equality operator with boolean literals
true == false;
---
(Program
  (Expression (Binary Equal (BooleanLiteral true) (BooleanLiteral false))))

=== Parse equality operator with nil
x == nil;
---
(Program
  (Expression (Binary Equal (Identifier x) (NilLiteral))))

=== Parse complex equality expression
x + 5 == y * 2;
---
(Program
  (Expression
    (Binary Equal
      (Binary Add (Identifier x) (NumericLiteral 5))
      (Binary Multiply (Identifier y) (NumericLiteral 2)))))

=== Parse chained equality expressions
x == y == true;
---
(Program
  (Expression
    (Binary Equal
      (Binary Equal (Identifier x) (Identifier y))
      (BooleanLiteral true))))

=== Parse equality with assignment
result = x == y;
---
(Program
  (Expression
    (Assignment Assign
      (Identifier result)
      (Binary Equal (Identifier x) (Identifier y)))))

=== Parse complex boolean expressions
isValid = (count > 0) == true;
---
(Program
  (Expression
    (Assignment Assign
      (Identifier isValid)
      (Binary Equal
        (Binary GreaterThan (Identifier count) (NumericLiteral 0))
        (BooleanLiteral true)))))

=== Parse greater than operator
x > 5;
---
(Program
  (Expression (Binary GreaterThan (Identifier x) (NumericLiteral 5))))

=== Parse greater than or equal to operator
x >= 10;
---
(Program
  (Expression (Binary GreaterThanOrEqualTo (Identifier x) (NumericLiteral 10))))

=== Parse less than operator
y < 20;
---
(Program
  (Expression (Binary LessThan (Identifier y) (NumericLiteral 20))))

=== Parse less than or equal to operator
y <= 30;
---
(Program
  (Expression (Binary LessThanOrEqualTo (Identifier y) (NumericLiteral 30))))

=== Parse relational operators with variables
x > y;
---
(Program
  (Expression (Binary GreaterThan (Identifier x) (Identifier y))))

=== Parse relational operators with expressions
x + 5 > y * 2;
---
(Program
  (Expression
    (Binary GreaterThan
      (Binary Add (Identifier x) (NumericLiteral 5))
      (Binary Multiply (Identifier y) (NumericLiteral 2)))))

=== Parse chained relational expressions
x > 5 > y;
---
(Program
  (Expression
    (Binary GreaterThan
      (Binary GreaterThan (Identifier x) (NumericLiteral 5))
      (Identifier y))))

=== Parse relational operators with parenthesized expressions
(x + 10) > (y - 5);
---
(Program
  (Expression
    (Binary GreaterThan
      (Binary Add (Identifier x) (NumericLiteral 10))
      (Binary Subtract (Identifier y) (NumericLiteral 5)))))

=== Parse relational operators precedence with assignment
result = x > y;
---
(Program
  (Expression
    (Assignment Assign
      (Identifier result)
      (Binary GreaterThan (Identifier x) (Identifier y)))))

=== Parse logical and operator
x && y;
---
(Program
  (Expression (Logical And (Identifier x) (Identifier y))))

=== Parse logical or operator
x || y;
---
(Program
  (Expression (Logical Or (Identifier x) (Identifier y))))

=== Parse chained logical and operators
a && b && c;
---
(Program
  (Expression
    (Logical And (Logical And (Identifier a) (Identifier b)) (Identifier c))))

=== Parse chained logical or operators
a || b || c;
---
(Program
  (Expression
    (Logical Or (Logical Or (Identifier a) (Identifier b)) (Identifier c))))

=== Parse mixed logical operators
a && b || c;
---
(Program
  (Expression
    (Logical Or (Logical And (Identifier a) (Identifier b)) (Identifier c))))

=== Parse logical operators with parentheses
a && (b || c);
---
(Program
  (Expression
    (Logical And (Identifier a) (Logical Or (Identifier b) (Identifier c)))))

=== Parse logical operators with boolean literals
true && false;
---
(Program
  (Expression (Logical And (BooleanLiteral true) (BooleanLiteral false))))

=== Parse logical operators with comparison expressions
x > 5 && y < 10;
---
(Program
  (Expression
    (Logical And
      (Binary GreaterThan (Identifier x) (NumericLiteral 5))
      (Binary LessThan (Identifier y) (NumericLiteral 10)))))

=== Parse logical operators with equality expressions
x == 5 || y != 10;
---
(Program
  (Expression
    (Logical Or
      (Binary Equal (Identifier x) (NumericLiteral 5))
      (Binary NotEqual (Identifier y) (NumericLiteral 10)))))

=== Parse complex logical expression
(a > b && c < d) || (e == f && g != h);
---
(Program
  (Expression
    (Logical Or
      (Logical And
        (Binary GreaterThan (Identifier a) (Identifier b))
        (Binary LessThan (Identifier c) (Identifier d)))
      (Logical And
        (Binary Equal (Identifier e) (Identifier f))
        (Binary NotEqual (Identifier g) (Identifier h))))))

=== Parse assignment with logical expression
result = a && b || c;
---
(Program
  (Expression
    (Assignment Assign
      (Identifier result)
      (Logical Or (Logical And (Identifier a) (Identifier b)) (Identifier c)))))

=== Parse variable initialization with logical expression
let isValid: boolean = x > 0 && x < 100;
---
(Program
  (VariableDeclaration
    (Variable
      (Identifier isValid)
      Boolean
      (Logical And
        (Binary GreaterThan (Identifier x) (NumericLiteral 0))
        (Binary LessThan (Identifier x) (NumericLiteral 100))))))

=== Parse logical expression in if condition
if (x > 0 && x < 100) {
    y = 1;
} else {
    y = 0;
}
---
(Program
  (If
    (Logical And
      (Binary GreaterThan (Identifier x) (NumericLiteral 0))
      (Binary LessThan (Identifier x) (NumericLiteral 100)))
    (Block
      (Expression (Assignment Assign (Identifier y) (NumericLiteral 1))))
    (Block
      (Expression (Assignment Assign (Identifier y) (NumericLiteral 0))))))
