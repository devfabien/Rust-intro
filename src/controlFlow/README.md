# Control Flow in Rust

Control flow in Rust is used to manage the execution of code based on certain conditions and loops. This guide covers the basic control flow constructs in Rust.

## Conditional Statements

### `if` Expressions

The `if` expression allows you to branch your code based on conditions.

```rust
let number = 5;

if number < 10 {
    println!("The number is less than 10");
} else {
    println!("The number is 10 or greater");
}
```

### `else if` and `else`

You can chain multiple conditions using `else if` and provide a fallback with `else`.

```rust
let number = 6;

if number % 4 == 0 {
    println!("The number is divisible by 4");
} else if number % 3 == 0 {
    println!("The number is divisible by 3");
} else if number % 2 == 0 {
    println!("The number is divisible by 2");
} else {
    println!("The number is not divisible by 4, 3, or 2");
}
```

## Looping Constructs

### `loop`

The `loop` keyword creates an infinite loop.

```rust
loop {
    println!("This will print forever!");
}
```

### `while` Loop

The `while` loop runs as long as a condition is true.

```rust
let mut number = 3;

while number != 0 {
    println!("{}!", number);
    number -= 1;
}
println!("LIFTOFF!!!");
```

### `for` Loop

The `for` loop is used to iterate over a collection.

```rust
let a = [10, 20, 30, 40, 50];

for element in a.iter() {
    println!("The value is: {}", element);
}
```


### `break` and `continue`

#### `break`

The `break` keyword is used to exit a loop immediately.

```rust
let mut counter = 0;

loop {
    counter += 1;

    if counter == 5 {
        break;
    }
}

println!("The loop stopped at counter = {}", counter);
```

#### `continue`

The `continue` keyword is used to skip the rest of the current iteration and move to the next iteration of the loop.

```rust
for number in 1..10 {
    if number % 2 == 0 {
        continue;
    }

    println!("Odd number: {}", number);
}
```


## `match` Expressions

The `match` expression is a powerful control flow operator that allows you to compare a value against a series of patterns.

```rust
let number = 1;

match number {
    1 => println!("One!"),
    2 => println!("Two!"),
    3 => println!("Three!"),
    _ => println!("Something else!"),
}
```

## Conclusion

Understanding control flow is essential for writing effective Rust programs. By using `if`, `else if`, `else`, `loop`, `while`, `for`, and `match`, you can control the execution of your code based on various conditions and patterns.

For more detailed information, refer to the [Rust documentation](https://doc.rust-lang.org/book/ch03-05-control-flow.html).