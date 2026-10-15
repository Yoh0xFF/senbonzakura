=== Parse simple class declaration
class Person {
  // Empty class
}
---
(Program
  (ClassDeclaration (Identifier Person) _ (Block)))

=== Parse class with methods
class Person {
  def getName() {
    return "John";
  }

  def getAge() {
    return 30;
  }
}
---
(Program
  (ClassDeclaration
    (Identifier Person)
    _
    (Block
      (FunctionDeclaration
        (Identifier getName)
        (Parameters)
        Void
        (Block
          (Return (StringLiteral "John"))))
      (FunctionDeclaration
        (Identifier getAge)
        (Parameters)
        Void
        (Block
          (Return (NumericLiteral 30)))))))

=== Parse class with inheritance
class Student extends Person {
  def getGrade() {
    return "A";
  }
}
---
(Program
  (ClassDeclaration
    (Identifier Student)
    (Identifier Person)
    (Block
      (FunctionDeclaration
        (Identifier getGrade)
        (Parameters)
        Void
        (Block
          (Return (StringLiteral "A")))))))

=== Parse class with constructor-like method
class Person {
  def constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }

  def getName() {
    return this.name;
  }

  def getAge() {
    return this.age;
  }
}
---
(Program
  (ClassDeclaration
    (Identifier Person)
    _
    (Block
      (FunctionDeclaration
        (Identifier constructor)
        (Parameters
          (Parameter (Identifier name) String)
          (Parameter (Identifier age) Number))
        Void
        (Block
          (Expression
            (Assignment Assign
              (Member (This) (Identifier name))
              (Identifier name)))
          (Expression
            (Assignment Assign
              (Member (This) (Identifier age))
              (Identifier age)))))
      (FunctionDeclaration
        (Identifier getName)
        (Parameters)
        Void
        (Block
          (Return (Member (This) (Identifier name)))))
      (FunctionDeclaration
        (Identifier getAge)
        (Parameters)
        Void
        (Block
          (Return (Member (This) (Identifier age))))))))

=== Parse class with instance variables
class Rectangle {
  def constructor(width: number, height: number) {
    this.width = width;
    this.height = height;
  }

  def getArea(): number {
    return this.width * this.height;
  }
}
---
(Program
  (ClassDeclaration
    (Identifier Rectangle)
    _
    (Block
      (FunctionDeclaration
        (Identifier constructor)
        (Parameters
          (Parameter (Identifier width) Number)
          (Parameter (Identifier height) Number))
        Void
        (Block
          (Expression
            (Assignment Assign
              (Member (This) (Identifier width))
              (Identifier width)))
          (Expression
            (Assignment Assign
              (Member (This) (Identifier height))
              (Identifier height)))))
      (FunctionDeclaration
        (Identifier getArea)
        (Parameters)
        Number
        (Block
          (Return
            (Binary Multiply
              (Member (This) (Identifier width))
              (Member (This) (Identifier height)))))))))

=== Parse class with super calls
class Child extends Parent {
  def constructor(name: string, age: number) {
    super(name);
    this.age = age;
  }

  def describe(): string {
    return super.getName() + " is " + this.age + " years old";
  }
}
---
(Program
  (ClassDeclaration
    (Identifier Child)
    (Identifier Parent)
    (Block
      (FunctionDeclaration
        (Identifier constructor)
        (Parameters
          (Parameter (Identifier name) String)
          (Parameter (Identifier age) Number))
        Void
        (Block
          (Expression (Call (Super) (Identifier name)))
          (Expression
            (Assignment Assign
              (Member (This) (Identifier age))
              (Identifier age)))))
      (FunctionDeclaration
        (Identifier describe)
        (Parameters)
        String
        (Block
          (Return
            (Binary Add
              (Binary Add
                (Binary Add
                  (Call (Member (Super) (Identifier getName)))
                  (StringLiteral " is "))
                (Member (This) (Identifier age)))
              (StringLiteral " years old"))))))))

=== Parse multiple classes
class Animal {
  def makeSound(): string {
    return "Generic animal sound";
  }
}

class Dog extends Animal {
  def makeSound(): string {
    return "Woof";
  }
}
---
(Program
  (ClassDeclaration
    (Identifier Animal)
    _
    (Block
      (FunctionDeclaration
        (Identifier makeSound)
        (Parameters)
        String
        (Block
          (Return (StringLiteral "Generic animal sound"))))))
  (ClassDeclaration
    (Identifier Dog)
    (Identifier Animal)
    (Block
      (FunctionDeclaration
        (Identifier makeSound)
        (Parameters)
        String
        (Block
          (Return (StringLiteral "Woof")))))))

=== Parse class with method using if statement
class Calculator {
  def max(a: number, b: number): number {
    if (a > b) {
      return a;
    } else {
      return b;
    }
  }
}
---
(Program
  (ClassDeclaration
    (Identifier Calculator)
    _
    (Block
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
              (Return (Identifier b)))))))))

=== Parse class with method using loops
class Summation {
  def sum(n: number): number {
    let result: number = 0;
    for (let i: number = 1; i <= n; i = i + 1) {
      result = result + i;
    }
    return result;
  }
}
---
(Program
  (ClassDeclaration
    (Identifier Summation)
    _
    (Block
      (FunctionDeclaration
        (Identifier sum)
        (Parameters (Parameter (Identifier n) Number))
        Number
        (Block
          (VariableDeclaration
            (Variable (Identifier result) Number (NumericLiteral 0)))
          (For
            (VariableDeclaration
              (Variable (Identifier i) Number (NumericLiteral 1)))
            (Binary LessThanOrEqualTo (Identifier i) (Identifier n))
            (Assignment Assign
              (Identifier i)
              (Binary Add (Identifier i) (NumericLiteral 1)))
            (Block
              (Expression
                (Assignment Assign
                  (Identifier result)
                  (Binary Add (Identifier result) (Identifier i))))))
          (Return (Identifier result)))))))

=== Parse class with method using function calls
class MathUtils {
  def factorial(n: number): number {
    if (n <= 1) {
      return 1;
    }
    return n * this.factorial(n - 1);
  }
}
---
(Program
  (ClassDeclaration
    (Identifier MathUtils)
    _
    (Block
      (FunctionDeclaration
        (Identifier factorial)
        (Parameters (Parameter (Identifier n) Number))
        Number
        (Block
          (If
            (Binary LessThanOrEqualTo (Identifier n) (NumericLiteral 1))
            (Block
              (Return (NumericLiteral 1))))
          (Return
            (Binary Multiply
              (Identifier n)
              (Call
                (Member (This) (Identifier factorial))
                (Binary Subtract (Identifier n) (NumericLiteral 1))))))))))

=== Parse class instantiation
class Point {
  def constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }
}

let p: Point = new Point(10, 20);
---
(Program
  (ClassDeclaration
    (Identifier Point)
    _
    (Block
      (FunctionDeclaration
        (Identifier constructor)
        (Parameters
          (Parameter (Identifier x) Number)
          (Parameter (Identifier y) Number))
        Void
        (Block
          (Expression
            (Assignment Assign (Member (This) (Identifier x)) (Identifier x)))
          (Expression
            (Assignment Assign (Member (This) (Identifier y)) (Identifier y)))))))
  (VariableDeclaration
    (Variable
      (Identifier p)
      (Class Point)
      (New (Identifier Point) (NumericLiteral 10) (NumericLiteral 20)))))

=== Parse empty class with extends
class Empty extends Base {
}
---
(Program
  (ClassDeclaration (Identifier Empty) (Identifier Base) (Block)))
