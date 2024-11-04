fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    // change(&s1);: this will give an error
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// when you try to modify the value of a reference, you'll get a compile-time error
// because references are immutable by default

fn change(some_string: &String) {
    some_string.push_str(", world");
}
