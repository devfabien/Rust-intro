# Rust-intro
# Introduction to Rust

Rust is a systems programming language focused on safety, speed, and concurrency. It achieves memory safety without needing a garbage collector, making it a popular choice for performance-critical applications.

## Rust Variables

In Rust, variables are immutable by default. To make a variable mutable, you use the `mut` keyword. Here's an example:

```rust
fn main() {
    let x = 5; // immutable variable
    let mut y = 10; // mutable variable
    println!("x: {x}, y: {y}");
    y += 5;
    println!("Updated y: {y}");
}
```

This code demonstrates how to declare and use both immutable and mutable variables in Rust.

If you don't specify the `mut` keyword for a variable that you later try to modify, the Rust compiler will throw an error. For example:

```rust
fn main() {
    let x = 5; // immutable variable
    x += 1; // error: cannot assign twice to immutable variable `x`
}
```

In this case, the compiler will produce an error message similar to:

```
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:3:5
  |
2 |     let x = 5; // immutable variable
  |         -
  |         |
  |         first assignment to `x`
3 |     x += 1; // error: cannot assign twice to immutable variable `x`
  |     ^^^^^^ cannot assign twice to immutable variable
```
## Rust Constants

Constants in Rust are similar to immutable variables, but there are a few key differences. Constants are always immutable, and their values must be set at compile time. They are declared using the `const` keyword and must have an explicit type. Constants can be declared in any scope, including the global scope.

Here's an example of declaring and using constants in Rust:

```rust
const MAX_POINTS: u32 = 100_000;

fn main() {
  println!("The maximum points are: {}", MAX_POINTS);
}
```

### Differences Between Constants and Variables

1. **Mutability**:
   - Variables can be mutable or immutable.
   - Constants are always immutable.

2. **Scope**:
   - Variables are scoped to the block they are declared in.
   - Constants can be declared in any scope, including the global scope.

3. **Initialization**:
   - Variables can be initialized at runtime.
   - Constants must be initialized at compile time.

4. **Type Annotation**:
   - Variables can have their types inferred.
   - Constants require explicit type annotations.

These differences make constants useful for values that are known at compile time and should not change throughout the execution of the program.
## Shadowing in Rust

Shadowing allows you to declare a new variable with the same name as a previous variable. The new variable shadows the previous one, and you can reuse the same name without mutability. This can be useful for transforming a value while keeping the variable name.

Here's an example of shadowing in Rust:

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```

In this example, the variable `x` is shadowed twice. The final value of `x` is 12.

### Differences Between Shadowing and Mutability

1. **Rebinding**:
   - Shadowing allows you to rebind a variable name to a new value.
   - Mutability allows you to change the value of a variable without rebinding.

2. **Type Changes**:
   - Shadowing allows the variable to change types.
   - Mutability does not allow type changes.

3. **Scope**:
   - Shadowed variables are scoped to the block they are declared in.
   - Mutable variables retain their scope.

Here's an example demonstrating type change with shadowing:

```rust
fn main() {
  let spaces = "   "; // spaces is a string
  let spaces = spaces.len(); // spaces is now an integer
  println!("The number of spaces is: {spaces}");
}
```

In this example, the variable `spaces` is first a string and then shadowed to become an integer.