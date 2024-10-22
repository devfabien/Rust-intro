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