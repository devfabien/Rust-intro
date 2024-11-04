// Mutable References in Rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("The modified string is: {}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// The &mut s syntax lets us create a mutable reference to s
// which allows us to modify s
// But mutable references have one big restriction: you can only have one mutable reference to a particular piece of data in a particular scope

// This code will not compile because it has two mutable references to s in the same scope:
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}

// This code will not compile because it has a mutable reference and an immutable reference in the same scope:
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;

    println!("{}, {}, and {}", r1, r2, r3);
}
