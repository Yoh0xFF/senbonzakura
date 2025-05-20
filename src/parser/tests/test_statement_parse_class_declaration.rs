use super::internal_util::execute;

#[test]
fn test_simple_class_declaration() {
    execute(
        r#"
        class Person {
            // Empty class
        }
        "#,
        r#"
        (program
            (class
                (id Person)
                (block)))
        "#,
    )
}

#[test]
fn test_class_with_methods() {
    execute(
        r#"
        class Person {
            def getName() {
                return "John";
            }

            def getAge() {
                return 30;
            }
        }
        "#,
        r#"
        (program
            (class
                (id Person)
                (block
                    (def
                        (id getName)
                        (return_type "void")
                        (block
                            (return (string "John"))))
                    (def
                        (id getAge)
                        (return_type "void")
                        (block
                            (return (number 30)))))))
        "#,
    )
}

#[test]
fn test_class_with_inheritance() {
    execute(
        r#"
        class Student extends Person {
            def getGrade() {
                return "A";
            }
        }
        "#,
        r#"
        (program
            (class
                (id Student)
                (extends (id Person))
                (block
                    (def
                        (id getGrade)
                        (return_type "void")
                        (block
                            (return (string "A")))))))
        "#,
    )
}

#[test]
fn test_class_with_constructor_like_method() {
    execute(
        r#"
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
        "#,
        r#"
        (program
            (class
                (id Person)
                (block
                    (def
                        (id constructor)
                        (params
                            (param (id name) (type String))
                            (param (id age) (type Number)))
                        (return_type "void")
                        (block
                            (expr (assign "=" (member "static" (this) (id name)) (id name)))
                            (expr (assign "=" (member "static" (this) (id age)) (id age)))))
                    (def
                        (id getName)
                        (return_type "void")
                        (block
                            (return (member "static" (this) (id name)))))
                    (def
                        (id getAge)
                        (return_type "void")
                        (block
                            (return (member "static" (this) (id age))))))))
        "#,
    )
}

#[test]
fn test_class_with_instance_variables() {
    execute(
        r#"
        class Rectangle {
            def constructor(width: number, height: number) {
                this.width = width;
                this.height = height;
            }

            def getArea(): number {
                return this.width * this.height;
            }
        }
        "#,
        r#"
        (program
            (class
                (id Rectangle)
                (block
                    (def
                        (id constructor)
                        (params
                            (param (id width) (type Number))
                            (param (id height) (type Number)))
                        (return_type "void")
                        (block
                            (expr (assign "=" (member "static" (this) (id width)) (id width)))
                            (expr (assign "=" (member "static" (this) (id height)) (id height)))))
                    (def
                        (id getArea)
                        (return_type Number)
                        (block
                            (return (binary "*" (member "static" (this) (id width)) (member "static" (this) (id height)))))))))
        "#,
    )
}

#[test]
fn test_class_with_super_calls() {
    execute(
        r#"
        class Child extends Parent {
            def constructor(name: string, age: number) {
                super(name);
                this.age = age;
            }

            def describe(): string {
                return super.getName() + " is " + this.age + " years old";
            }
        }
        "#,
        r#"
        (program
            (class
                (id Child)
                (extends (id Parent))
                (block
                    (def
                        (id constructor)
                        (params
                            (param (id name) (type String))
                            (param (id age) (type Number)))
                        (return_type "void")
                        (block
                            (expr (call (super) (args (id name))))
                            (expr (assign "=" (member "static" (this) (id age)) (id age)))))
                    (def
                        (id describe)
                        (return_type String)
                        (block
                            (return (binary "+" (binary "+" (binary "+" (call (member "static" (super) (id getName))) (string " is ")) (member "static" (this) (id age))) (string " years old"))))))))
        "#,
    )
}

#[test]
fn test_multiple_classes() {
    execute(
        r#"
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
        "#,
        r#"
        (program
            (class
                (id Animal)
                (block
                    (def
                        (id makeSound)
                        (return_type String)
                        (block
                            (return (string "Generic animal sound"))))))
            (class
                (id Dog)
                (extends (id Animal))
                (block
                    (def
                        (id makeSound)
                        (return_type String)
                        (block
                            (return (string "Woof")))))))
        "#,
    )
}

#[test]
fn test_class_with_method_using_if_statement() {
    execute(
        r#"
        class Calculator {
            def max(a: number, b: number): number {
                if (a > b) {
                    return a;
                } else {
                    return b;
                }
            }
        }
        "#,
        r#"
        (program
            (class
                (id Calculator)
                (block
                    (def
                        (id max)
                        (params
                            (param (id a) (type Number))
                            (param (id b) (type Number)))
                        (return_type Number)
                        (block
                            (if
                                (binary ">" (id a) (id b))
                                (block
                                    (return (id a)))
                                (block
                                    (return (id b)))))))))
        "#,
    )
}

#[test]
fn test_class_with_method_using_loops() {
    execute(
        r#"
        class Summation {
            def sum(n: number): number {
                let result: number = 0;
                for (let i: number = 1; i <= n; i = i + 1) {
                    result = result + i;
                }
                return result;
            }
        }
        "#,
        r#"
        (program
            (class
                (id Summation)
                (block
                    (def
                        (id sum)
                        (params
                            (param (id n) (type Number)))
                        (return_type Number)
                        (block
                            (let
                                (init
                                    (id result)
                                    (type Number)
                                    (number 0)))
                            (for
                                (let
                                    (init
                                        (id i)
                                        (type Number)
                                        (number 1)))
                                (binary "<=" (id i) (id n))
                                (assign "=" (id i) (binary "+" (id i) (number 1)))
                                (block
                                    (expr (assign "=" (id result) (binary "+" (id result) (id i))))))
                            (return (id result)))))))
        "#,
    )
}

#[test]
fn test_class_with_method_using_function_calls() {
    execute(
        r#"
        class MathUtils {
            def factorial(n: number): number {
                if (n <= 1) {
                    return 1;
                }
                return n * this.factorial(n - 1);
            }
        }
        "#,
        r#"
        (program
            (class
                (id MathUtils)
                (block
                    (def
                        (id factorial)
                        (params
                            (param (id n) (type Number)))
                        (return_type Number)
                        (block
                            (if
                                (binary "<=" (id n) (number 1))
                                (block
                                    (return (number 1))))
                            (return (binary "*" (id n) (call (member "static" (this) (id factorial)) (args (binary "-" (id n) (number 1)))))))))))
        "#,
    )
}

#[test]
fn test_class_instantiation() {
    execute(
        r#"
        class Point {
            def constructor(x: number, y: number) {
                this.x = x;
                this.y = y;
            }
        }

        let p: Point = new Point(10, 20);
        "#,
        r#"
        (program
            (class
                (id Point)
                (block
                    (def
                        (id constructor)
                        (params
                            (param (id x) (type Number))
                            (param (id y) (type Number)))
                        (return_type "void")
                        (block
                            (expr (assign "=" (member "static" (this) (id x)) (id x)))
                            (expr (assign "=" (member "static" (this) (id y)) (id y)))))))
            (let
                (init
                    (id p)
                    (type(class Point))
                    (new (id Point) (args (number 10) (number 20))))))
        "#,
    )
}

#[test]
fn test_empty_class_with_extends() {
    execute(
        r#"
        class Empty extends Base {
        }
        "#,
        r#"
        (program
            (class
                (id Empty)
                (extends (id Base))
                (block)))
        "#,
    )
}
