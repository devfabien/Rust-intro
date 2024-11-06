fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);
}

// With Rust’s .. range syntax, if you want to start at index 0, you can drop the value before the two periods. In other words, these are equal:
// let slice = &s[0..2];
// let slice = &s[..2];
