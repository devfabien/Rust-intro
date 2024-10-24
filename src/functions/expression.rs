fn main() {
    let y = {
        let x = 3; // This block is a statement
        x + 1 // This block is an expression that evaluates to 4
    };

    println!("The value of y is: {y}");
}
