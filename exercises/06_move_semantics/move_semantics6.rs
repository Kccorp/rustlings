// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

//WRITE UP: change the function signature to take ownership. as we know & is a reference and it does not take ownership so we need
// to switch in the function signature to take ownership of the string. & = no ownership

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);
    string_uppercase(data);

}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
