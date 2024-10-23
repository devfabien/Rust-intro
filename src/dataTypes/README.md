This module demonstrates the usage of various data types in Rust.
# Data Types
Rust is a statically typed language, which means that it must know the types of all variables at compile time. 
The main data types in Rust are:





- **Scalar Types**: Represent a single value.
  - `integer`: Numbers without a fractional component. Examples include `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`, `i128`, `u128`, and `isize`, `usize`.

    Integer Types in Rust In Rust, integers can be either signed or unsigned

    - Signed Integers (i): These can represent both positive and negative numbers. The range of values they can hold is from -(2^(n-1)) to 2^(n-1) - 1, where n is the number of bits.

    - Unsigned Integers (u): These can only represent non-negative numbers. The range of values they can hold is from 0 to 2^n - 1, where n is the number of bits.

  - `floating-point`: Numbers with a fractional component. Examples include `f32` and `f64`.

    - The default type is f64 because on modern CPUs, it’s roughly the same speed as f32 but is capable of more precision. All floating-point types are signed.

    - Floating-point numbers are represented according to the IEEE-754 standard. The f32 type is a single-precision float, and f64 has double precision.

  - `char`: A single Unicode scalar value. Example: `'a'`.
  - `bool`: A boolean value. Can be either `true` or `false`.

- **Compound Types**: Can group multiple values into one type.
  - `tuple`: A fixed-size collection of values of different types. Example: `(i32, f64, char)`.
  - `array`: A fixed-size collection of values of the same type. Example: `[i32; 5]`.


## Examples
```rust
// Integer
let x: i32 = 42;
// Floating-point
let y: f64 = 3.14;
// Character
let z: char = 'A';
// Boolean
let is_active: bool = true;
// Tuple
let tuple: (i32, f64, char) = (42, 3.14, 'A');
// Array
let array: [i32; 3] = [1, 2, 3];
```