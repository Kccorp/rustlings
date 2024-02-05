// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

// WHITE UP: mismatched types error is caused by the return type of the square function. fixed by deleted the semicolon
// line 14. The return type of the function is i32, so the return value should be an i32, not a unit type and expression
// without a semicolon returns a value.

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}
