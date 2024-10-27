# Functions in Rust

This document provides an overview of functions in Rust, including examples and explanations of their usage.

## Table of Contents

- [Introduction](#introduction)
- [Function Definitions](#function-definitions)
- [Examples](#examples)

## Introduction

Functions are a fundamental building block in Rust. They allow you to encapsulate code into reusable units. Functions in Rust are defined using the `fn` keyword, followed by the function name, parameters, and body.


## Parameters, Statements, and Expressions in Rust

### Adding Two Numbers

This function takes two integers and returns their sum.

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

#### Arguments

- `a` - An integer representing the first addend.
- `b` - An integer representing the second addend.

#### Returns

- An integer representing the sum of `a` and `b`.

#### Examples

```rust
let result = add(5, 3);
assert_eq!(result, 8);
```

### Calculating the Length of a String

This function takes a string slice and returns its length.

```rust
fn length_of_string(s: &str) -> usize {
    s.len()
}
```

#### Arguments

- `s` - A string slice whose length is to be calculated.

#### Returns

- A usize representing the length of the string slice.

#### Examples

```rust
let length = length_of_string("hello");
assert_eq!(length, 5);
```

## Function Definitions

## Examples

Here are some examples of how to use the functions defined above.

```rust
fn main() {
    let sum = add(10, 20);
    println!("The sum is: {}", sum);

    let length = length_of_string("Rust");
    println!("The length is: {}", length);
}
```

This code will output:

```
The sum is: 30
The length is: 4
```


### Parameters

Parameters are the inputs to a function. They are specified in the function signature inside parentheses. Each parameter must have a type specified.

Example:

```rust
fn example_function(param1: i32, param2: &str) {
    // function body
}
```

In this example, `param1` is an integer and `param2` is a string slice.

### Statements and Expressions

Rust distinguishes between statements and expressions. A statement performs an action but does not return a value, while an expression evaluates to a value.

#### Statements

Statements are instructions that perform some action and do not return a value. Examples include variable declarations and function calls.

Example:

```rust
let x = 5; // This is a statement
```

#### Expressions

Expressions evaluate to a value and can be used in statements. Most things in Rust are expressions, including function calls, arithmetic operations, and blocks.

Example:

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1 // This block is an expression that evaluates to 4
    };

    println!("The value of y is: {y}");
}
```
 Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. Keep this in mind as you explore function return values and expressions next.

### Example Function with Parameters, Statements, and Expressions

```rust
fn factorial(n: i32) -> i32 {
    if n < 0 {
        return -1; // Error indicator for negative input
    } else if n == 0 {
        return 1; // Base case: 0! is 1
    } else {
        (1..=n).product() // Expression that calculates the factorial
    }
}
```

#### Arguments

- `n` - An integer representing the number for which the factorial is to be calculated. Must be a non-negative integer.

#### Returns

- An integer representing the factorial of the given number. If the input is 0, returns 1. If the input is a negative number, returns -1 as an error indicator.

#### Examples

```rust
let result = factorial(5);
assert_eq!(result, 120);

let error_result = factorial(-3);
assert_eq!(error_result, -1);
```
## Function Return Values

In Rust, functions can return values using the `->` syntax followed by the return type. The return value is the last expression in the function body, and it does not require a semicolon.

In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly

### Example: Returning a Value

This function returns the square of an integer.

```rust
fn square(x: i32) -> i32 {
    x * x
}
```

#### Arguments

- `x` - An integer to be squared.

#### Returns

- An integer representing the square of `x`.

#### Examples

```rust
let result = square(4);
assert_eq!(result, 16);
```

### Example: Returning Multiple Values

Rust supports returning multiple values using tuples.

```rust
fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}
```

#### Arguments

- `a` - An integer.
- `b` - Another integer.

#### Returns

- A tuple containing the smaller and larger of the two integers.

#### Examples

```rust
let (min, max) = min_max(3, 7);
assert_eq!(min, 3);
assert_eq!(max, 7);
```

### Example: Early Return

You can return early from a function using the `return` keyword.

```rust
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None; // Early return for division by zero
    }
    Some(a / b)
}
```

#### Arguments

- `a` - The dividend.
- `b` - The divisor.

#### Returns

- `Some` containing the quotient if `b` is not zero, otherwise `None`.

#### Examples

```rust
let result = divide(10, 2);
assert_eq!(result, Some(5));

let error_result = divide(10, 0);
assert_eq!(error_result, None);
```
