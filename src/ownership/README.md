# Ownership in Rust

Ownership is a central feature of Rust that enables memory safety without needing a garbage collector. Here are the key concepts:

## Key Concepts

### 1. Ownership Rules
1. Each value in Rust has a variable that's called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

### 2. Borrowing
- **Immutable Borrowing**: You can have multiple immutable references (`&T`) to a value.
- **Mutable Borrowing**: You can have only one mutable reference (`&mut T`) to a value at a time.

### 3. Slices
Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.

### 4. References and Dereferencing
References allow you to refer to some value without taking ownership of it. Dereferencing is used to access the value that a reference points to.

## Examples

### Ownership Transfer
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is no longer valid
    println!("{}", s2);
}
```

### Borrowing
```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Borrowing s1
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

### Mutable Borrowing
```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s); // Mutable borrow
    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

### Slices
```rust
fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);
}
```

## Conclusion
Understanding ownership is crucial for writing safe and efficient Rust code. It ensures memory safety and prevents data races at compile time.

For more detailed information, refer to the [Rust Book](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html).
