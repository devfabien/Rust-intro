fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);
}

// With Rust’s .. range syntax, if you want to start at index 0, you can drop the value before the two periods. In other words, these are equal:
// let slice = &s[0..2];
// let slice = &s[..2];

// If you want to start the range at the first index, you can drop the value before the two periods. These are equal:
// let len = s.len();
// let slice = &s[3..len];
// let slice = &s[3..];

// You can also use .. to include all values in a slice. These are equal:
// let len = s.len();
// let slice = &s[0..len];
// let slice = &s[..];
fn _first_space(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn _first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
