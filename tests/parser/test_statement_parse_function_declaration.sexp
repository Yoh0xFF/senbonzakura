=== Parse simple function declaration
def myFunction() {
  x = 10;
}
---
(Program
  (FunctionDeclaration
    (Identifier myFunction)
    (Parameters)
    Void
    (Block
      (Expression (Assignment Assign (Identifier x) (NumericLiteral 10))))))

=== Parse function declaration with single parameter
def add(x: number): number {
  return x + 1;
}
---
(Program
  (FunctionDeclaration
    (Identifier add)
    (Parameters (Parameter (Identifier x) Number))
    Number
    (Block
      (Return (Binary Add (Identifier x) (NumericLiteral 1))))))

=== Parse function declaration with multiple parameters
def add(x: number, y: number, z: number): number {
  return x + y + z;
}
---
(Program
  (FunctionDeclaration
    (Identifier add)
    (Parameters
      (Parameter (Identifier x) Number)
      (Parameter (Identifier y) Number)
      (Parameter (Identifier z) Number))
    Number
    (Block
      (Return
        (Binary Add (Binary Add (Identifier x) (Identifier y)) (Identifier z))))))

=== Parse function declaration with empty body
def emptyFunction() {
}
---
(Program
  (FunctionDeclaration (Identifier emptyFunction) (Parameters) Void (Block)))

=== Parse function declaration with variable declaration
def initFunction(): number {
  let x: number = 10;
  let y: number = 20;
  return x + y;
}
---
(Program
  (FunctionDeclaration
    (Identifier initFunction)
    (Parameters)
    Number
    (Block
      (VariableDeclaration (Variable (Identifier x) Number (NumericLiteral 10)))
      (VariableDeclaration (Variable (Identifier y) Number (NumericLiteral 20)))
      (Return (Binary Add (Identifier x) (Identifier y))))))

=== Parse function declaration with if statement
def max(a: number, b: number): number {
  if (a > b) {
    return a;
  } else {
    return b;
  }
}
---
(Program
  (FunctionDeclaration
    (Identifier max)
    (Parameters
      (Parameter (Identifier a) Number)
      (Parameter (Identifier b) Number))
    Number
    (Block
      (If
        (Binary GreaterThan (Identifier a) (Identifier b))
        (Block
          (Return (Identifier a)))
        (Block
          (Return (Identifier b)))))))

=== Parse function declaration with while loop
def factorial(n: number): number {
  let result: number = 1;
  while (n > 1) {
    result = result * n;
    n = n - 1;
  }
  return result;
}
---
(Program
  (FunctionDeclaration
    (Identifier factorial)
    (Parameters (Parameter (Identifier n) Number))
    Number
    (Block
      (VariableDeclaration
        (Variable (Identifier result) Number (NumericLiteral 1)))
      (While
        (Binary GreaterThan (Identifier n) (NumericLiteral 1))
        (Block
          (Expression
            (Assignment Assign
              (Identifier result)
              (Binary Multiply (Identifier result) (Identifier n))))
          (Expression
            (Assignment Assign
              (Identifier n)
              (Binary Subtract (Identifier n) (NumericLiteral 1))))))
      (Return (Identifier result)))))

=== Parse function declaration with return type
def earlyReturn(x: number): number {
  if (x <= 0) {
    return 0;
  }
  return x;
}
---
(Program
  (FunctionDeclaration
    (Identifier earlyReturn)
    (Parameters (Parameter (Identifier x) Number))
    Number
    (Block
      (If
        (Binary LessThanOrEqualTo (Identifier x) (NumericLiteral 0))
        (Block
          (Return (NumericLiteral 0))))
      (Return (Identifier x)))))

=== Parse function declaration with complex expression
def evaluate(a: number, b: number, c: number): number {
  return a * b + c * (a + b);
}
---
(Program
  (FunctionDeclaration
    (Identifier evaluate)
    (Parameters
      (Parameter (Identifier a) Number)
      (Parameter (Identifier b) Number)
      (Parameter (Identifier c) Number))
    Number
    (Block
      (Return
        (Binary Add
          (Binary Multiply (Identifier a) (Identifier b))
          (Binary Multiply
            (Identifier c)
            (Binary Add (Identifier a) (Identifier b))))))))

=== Parse multiple function declarations
def add(a: number, b: number): number {
  return a + b;
}

def subtract(a: number, b: number): number {
  return a - b;
}
---
(Program
  (FunctionDeclaration
    (Identifier add)
    (Parameters
      (Parameter (Identifier a) Number)
      (Parameter (Identifier b) Number))
    Number
    (Block
      (Return (Binary Add (Identifier a) (Identifier b)))))
  (FunctionDeclaration
    (Identifier subtract)
    (Parameters
      (Parameter (Identifier a) Number)
      (Parameter (Identifier b) Number))
    Number
    (Block
      (Return (Binary Subtract (Identifier a) (Identifier b))))))
