// options3.rs
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    // 1.y is consumed here
    match y {
        // 3. so we only borrow it here
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
    }
    // 2. but still used here
    y; // Fix without deleting this line.
}
