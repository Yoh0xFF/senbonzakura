=== Parse simple member access with dot notation
obj.property;
---
(Program
  (Expression (Member (Identifier obj) (Identifier property))))

=== Parse computed member access with bracket notation
obj[property];
---
(Program
  (Expression (ComputedMember (Identifier obj) (Identifier property))))

=== Parse computed member access with numeric literal
obj[42];
---
(Program
  (Expression (ComputedMember (Identifier obj) (NumericLiteral 42))))

=== Parse computed member access with string literal
obj["property"];
---
(Program
  (Expression (ComputedMember (Identifier obj) (StringLiteral "property"))))

=== Parse computed member access with binary expression
obj[x + y];
---
(Program
  (Expression
    (ComputedMember (Identifier obj) (Binary Add (Identifier x) (Identifier y)))))

=== Parse chained member access with dot notation
obj.inner.property;
---
(Program
  (Expression
    (Member (Member (Identifier obj) (Identifier inner)) (Identifier property))))

=== Parse chained computed member access
obj[x][y];
---
(Program
  (Expression
    (ComputedMember
      (ComputedMember (Identifier obj) (Identifier x))
      (Identifier y))))

=== Parse mixed member access (dot and bracket notation)
obj.inner[index].property;
---
(Program
  (Expression
    (Member
      (ComputedMember
        (Member (Identifier obj) (Identifier inner))
        (Identifier index))
      (Identifier property))))

=== Parse member access with simple assignment
obj.property = 42;
---
(Program
  (Expression
    (Assignment Assign
      (Member (Identifier obj) (Identifier property))
      (NumericLiteral 42))))

=== Parse computed member access with assignment
obj[property] = 42;
---
(Program
  (Expression
    (Assignment Assign
      (ComputedMember (Identifier obj) (Identifier property))
      (NumericLiteral 42))))

=== Parse member access with complex assignment operator
obj.property += 42;
---
(Program
  (Expression
    (Assignment AssignAdd
      (Member (Identifier obj) (Identifier property))
      (NumericLiteral 42))))

=== Parse deep nested member access
obj.a.b.c.d.property;
---
(Program
  (Expression
    (Member
      (Member
        (Member
          (Member (Member (Identifier obj) (Identifier a)) (Identifier b))
          (Identifier c))
        (Identifier d))
      (Identifier property))))

=== Parse member access in binary expression
obj.value + 10;
---
(Program
  (Expression
    (Binary Add
      (Member (Identifier obj) (Identifier value))
      (NumericLiteral 10))))

=== Parse member access in logical expression
obj.valid && hasPermission;
---
(Program
  (Expression
    (Logical And
      (Member (Identifier obj) (Identifier valid))
      (Identifier hasPermission))))

=== Parse variable declaration with member access
let value: string = obj.property;
---
(Program
  (VariableDeclaration
    (Variable
      (Identifier value)
      String
      (Member (Identifier obj) (Identifier property)))))

=== Parse if statement with member access condition
if (user.isAdmin) {
  permission.granted = true;
}
---
(Program
  (If
    (Member (Identifier user) (Identifier isAdmin))
    (Block
      (Expression
        (Assignment Assign
          (Member (Identifier permission) (Identifier granted))
          (BooleanLiteral true))))))

=== Parse if statement with computed member access
if (permissions[role]) {
  access[level] = true;
}
---
(Program
  (If
    (ComputedMember (Identifier permissions) (Identifier role))
    (Block
      (Expression
        (Assignment Assign
          (ComputedMember (Identifier access) (Identifier level))
          (BooleanLiteral true))))))

=== Parse complex computed member access with arithmetic
matrix[i * 10 + j];
---
(Program
  (Expression
    (ComputedMember
      (Identifier matrix)
      (Binary Add
        (Binary Multiply (Identifier i) (NumericLiteral 10))
        (Identifier j)))))

=== Parse member access with unary NOT operation
!obj.enabled;
---
(Program
  (Expression (Unary Not (Member (Identifier obj) (Identifier enabled)))))

=== Parse simple function call with no arguments
foo();
---
(Program
  (Expression (Call (Identifier foo))))

=== Parse function call with single argument
foo(42);
---
(Program
  (Expression (Call (Identifier foo) (NumericLiteral 42))))

=== Parse function call with multiple arguments
foo(x, y, 42);
---
(Program
  (Expression
    (Call (Identifier foo) (Identifier x) (Identifier y) (NumericLiteral 42))))

=== Parse function call with expression argument
foo(x + y);
---
(Program
  (Expression
    (Call (Identifier foo) (Binary Add (Identifier x) (Identifier y)))))

=== Parse chained function calls
foo()();
---
(Program
  (Expression (Call (Call (Identifier foo)))))

=== Parse method call on object
obj.method();
---
(Program
  (Expression (Call (Member (Identifier obj) (Identifier method)))))

=== Parse method call using computed member access
obj['method']();
---
(Program
  (Expression (Call (ComputedMember (Identifier obj) (StringLiteral "method")))))

=== Parse complex nested function calls
foo(bar(), baz(42));
---
(Program
  (Expression
    (Call
      (Identifier foo)
      (Call (Identifier bar))
      (Call (Identifier baz) (NumericLiteral 42)))))

=== Parse function calls in binary expression
foo() + bar();
---
(Program
  (Expression (Binary Add (Call (Identifier foo)) (Call (Identifier bar)))))

=== Parse function calls in logical expression
isValid() && hasPermission();
---
(Program
  (Expression
    (Logical And (Call (Identifier isValid)) (Call (Identifier hasPermission)))))

=== Parse function call in assignment
result = getValue();
---
(Program
  (Expression
    (Assignment Assign (Identifier result) (Call (Identifier getValue)))))

=== Parse function call with complex nested expressions as arguments
calculate(x + y, z * 2, obj.property);
---
(Program
  (Expression
    (Call
      (Identifier calculate)
      (Binary Add (Identifier x) (Identifier y))
      (Binary Multiply (Identifier z) (NumericLiteral 2))
      (Member (Identifier obj) (Identifier property)))))

=== Parse variable declaration with function call
let result: string = getValue();
---
(Program
  (VariableDeclaration
    (Variable (Identifier result) String (Call (Identifier getValue)))))

=== Parse if statement with function call condition
if (isAdmin()) {
    grantAccess();
}
---
(Program
  (If
    (Call (Identifier isAdmin))
    (Block
      (Expression (Call (Identifier grantAccess))))))

=== Parse function call with boolean argument
setEnabled(true);
---
(Program
  (Expression (Call (Identifier setEnabled) (BooleanLiteral true))))

=== Parse function call with string argument
log("Hello, world!");
---
(Program
  (Expression (Call (Identifier log) (StringLiteral "Hello, world!"))))

=== Parse member access on call result
foo().bar;
---
(Program
  (Expression (Member (Call (Identifier foo)) (Identifier bar))))

=== Parse computed member access on call result
foo()[index];
---
(Program
  (Expression (ComputedMember (Call (Identifier foo)) (Identifier index))))

=== Parse member access on method call result
obj.getInner().property;
---
(Program
  (Expression
    (Member
      (Call (Member (Identifier obj) (Identifier getInner)))
      (Identifier property))))

=== Parse chained method calls
a.b().c();
---
(Program
  (Expression
    (Call (Member (Call (Member (Identifier a) (Identifier b))) (Identifier c)))))

=== Parse chained calls and member access
a().b().c();
---
(Program
  (Expression
    (Call (Member (Call (Member (Call (Identifier a)) (Identifier b))) (Identifier c)))))

=== Parse assignment to member of call result
foo().bar = 1;
---
(Program
  (Expression
    (Assignment Assign
      (Member (Call (Identifier foo)) (Identifier bar))
      (NumericLiteral 1))))
