=== Parse identifier expression
variableName;
---
(Program
  (Expression (Identifier variableName)))

=== Parse multiple identifiers
first;
second;
---
(Program
  (Expression (Identifier first))
  (Expression (Identifier second)))

=== Parse identifier starting with keyword
letVariable;
---
(Program
  (Expression (Identifier letVariable)))

=== Parse identifier with numbers
user123;
---
(Program
  (Expression (Identifier user123)))

=== Parse identifier with underscore
user_name;
---
(Program
  (Expression (Identifier user_name)))

=== Parse parenthesized expression
(42);
---
(Program
  (Expression (NumericLiteral 42)))

=== Parse nested parenthesized expression
((42));
---
(Program
  (Expression (NumericLiteral 42)))

=== Parse parenthesized identifier
(x);
---
(Program
  (Expression (Identifier x)))

=== Parse parenthesized binary expression
(x + y);
---
(Program
  (Expression (Binary Add (Identifier x) (Identifier y))))

=== Parse parenthesized complex expression
(a * b + c / d);
---
(Program
  (Expression
    (Binary Add
      (Binary Multiply (Identifier a) (Identifier b))
      (Binary Divide (Identifier c) (Identifier d)))))

=== Parse this expression
this;
---
(Program
  (Expression (This)))

=== Parse this with member access
this.property;
---
(Program
  (Expression (Member (This) (Identifier property))))

=== Parse this with computed member access
this[property];
---
(Program
  (Expression (ComputedMember (This) (Identifier property))))

=== Parse this with method call
this.method();
---
(Program
  (Expression (Call (Member (This) (Identifier method)))))

=== Parse super expression
super;
---
(Program
  (Expression (Super)))

=== Parse super with member access
super.property;
---
(Program
  (Expression (Member (Super) (Identifier property))))

=== Parse super with method call
super.method();
---
(Program
  (Expression (Call (Member (Super) (Identifier method)))))

=== Parse super with arguments
super(arg1, arg2);
---
(Program
  (Expression (Call (Super) (Identifier arg1) (Identifier arg2))))

=== Parse new expression no arguments
new Object();
---
(Program
  (Expression (New (Identifier Object))))

=== Parse new expression with arguments
new Point(10, 20);
---
(Program
  (Expression (New (Identifier Point) (NumericLiteral 10) (NumericLiteral 20))))

=== Parse new expression with member access
new namespace.Class();
---
(Program
  (Expression (New (Member (Identifier namespace) (Identifier Class)))))

=== Parse new expression with computed member access
new objects[member]();
---
(Program
  (Expression (New (ComputedMember (Identifier objects) (Identifier member)))))

=== Parse new expression with complex arguments
new Constructor(x + y, obj.method(), 42);
---
(Program
  (Expression
    (New
      (Identifier Constructor)
      (Binary Add (Identifier x) (Identifier y))
      (Call (Member (Identifier obj) (Identifier method)))
      (NumericLiteral 42))))

=== Parse primary expression in binary context
42 + (x * y);
---
(Program
  (Expression
    (Binary Add
      (NumericLiteral 42)
      (Binary Multiply (Identifier x) (Identifier y)))))

=== Parse primary expression in logical context
(a > b) && this.isValid();
---
(Program
  (Expression
    (Logical And
      (Binary GreaterThan (Identifier a) (Identifier b))
      (Call (Member (This) (Identifier isValid))))))

=== Parse primary expression in assignment
result = (a + b) * c;
---
(Program
  (Expression
    (Assignment Assign
      (Identifier result)
      (Binary Multiply
        (Binary Add (Identifier a) (Identifier b))
        (Identifier c)))))

=== Parse primary expression in initialization
let value: Vector = new Vector(1, 2, 3);
---
(Program
  (VariableDeclaration
    (Variable
      (Identifier value)
      (Class Vector)
      (New
        (Identifier Vector)
        (NumericLiteral 1)
        (NumericLiteral 2)
        (NumericLiteral 3)))))

=== Parse primary expression in conditional
if (this.isAdmin()) {
    return true;
}
---
(Program
  (If
    (Call (Member (This) (Identifier isAdmin)))
    (Block
      (Return (BooleanLiteral true)))))

=== Parse primary expression in loop
while (this.hasNext()) {
    this.process();
}
---
(Program
  (While
    (Call (Member (This) (Identifier hasNext)))
    (Block
      (Expression (Call (Member (This) (Identifier process)))))))

=== Parse mixed primary expressions
let result: number = (this.calculate(x, y) + super.getValue()) * new Factor(42).apply();
---
(Program
  (VariableDeclaration
    (Variable
      (Identifier result)
      Number
      (Binary Multiply
        (Binary Add
          (Call
            (Member (This) (Identifier calculate))
            (Identifier x)
            (Identifier y))
          (Call (Member (Super) (Identifier getValue))))
        (Call
          (Member
            (New (Identifier Factor) (NumericLiteral 42))
            (Identifier apply)))))))
