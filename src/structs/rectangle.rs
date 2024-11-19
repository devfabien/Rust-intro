// First method: using variables.
fn _main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        _area(width1, height1)
    );
}

fn _area(width: u32, height: u32) -> u32 {
    width * height
}

// Second method: using tuples.
fn _main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        _area(rect1)
    );
}

fn _area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
