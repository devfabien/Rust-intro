# Rust Structs

Structs in Rust are a way to create custom data types. They allow you to group related data together and define the structure of your data. (They are like object in javascript)

## Defining a Struct

To define a struct in Rust, use the `struct` keyword followed by the name of the struct and its fields:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

## Creating an Instance

You can create an instance of a struct by specifying values for each field:

```rust
let user1 = User {
    username: String::from("exampleuser"),
    email: String::from("user@example.com"),
    sign_in_count: 1,
    active: true,
};
```

## Accessing Struct Fields

You can access the fields of a struct using dot notation:

```rust
println!("Username: {}", user1.username);
println!("Email: {}", user1.email);
```

## Updating Structs

To create a new instance of a struct with some updated values, you can use the struct update syntax:

```rust
let user2 = User {
    email: String::from("newuser@example.com"),
    ..user1
};
```

## Tuple Structs

Rust also supports tuple structs, which are similar to regular tuples but have named types:

```rust
struct Color(i32, i32, i32);
let black = Color(0, 0, 0);
```

## Unit-Like Structs

 Unit-like structs are structs that do not have any fields. They are useful when you need to implement a trait on some type but don't need to store any data.
 
 These structs are similar to the unit type `()` in Rust, hence the name "unit-like".
 They can be used to create types that have no data but still need to implement certain traits or behaviors.
 
 Example:
 ```rust
 struct AlwaysEqual;
 
 impl PartialEq for AlwaysEqual {
     fn eq(&self, _: &Self) -> bool {
         true
     }
 }
 
 impl Eq for AlwaysEqual {}
 
 let a = AlwaysEqual;
 let b = AlwaysEqual;
 assert_eq!(a, b);
 ```
 In this example, `AlwaysEqual` is a unit-like struct that implements the `PartialEq` and `Eq` traits, always returning `true` for equality checks.

Unit-like structs are useful when you need to implement a trait on some type but don't need to store any data:

```rust
struct AlwaysEqual;
```

## Methods on Structs

You can define methods on structs using `impl` blocks. Methods are functions that are defined within the context of a struct and have access to the struct's fields.

### Defining Methods

Here is an example of how to define methods on a struct:

```rust
impl User {
    fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            sign_in_count: 1,
            active: true,
        }
    }

    fn deactivate(&mut self) {
        self.active = false;
    }

    fn reactivate(&mut self) {
        self.active = true;
    }
}
```

### Calling Methods

You can call methods on an instance of a struct using dot notation:

```rust
let mut user1 = User::new(String::from("exampleuser"), String::from("user@example.com"));
user1.deactivate();
println!("User active: {}", user1.active);
user1.reactivate();
println!("User active: {}", user1.active);
```

Methods provide a way to encapsulate behavior related to your data and make your code more modular and readable.

 ## Conclusion
        
Structs are a fundamental part of Rust that allow you to create complex data types and manage related data efficiently. They are versatile and can be used in various ways to suit your programming needs.