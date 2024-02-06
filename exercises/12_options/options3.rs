// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

//WRITE UP: We can use the ref keyword to match the reference of the point struct and print the co-ordinates.
// The ref keyword is used to create a reference to the value of the variable in the match statement so the value is not
// really moved, and it can be used after the match statement.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
