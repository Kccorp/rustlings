// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

//WRITE UP: We need to call the appropriate function for each value.
// "" is a string slice by design
// to_sting() returns a String
// String::from("hi") create a String from a string slice
// to_owned() returns a String
// into() returns a String
// format!() convert to a String
// &String::from("abc")[0..1] returns a string slice because we are using the & operator to get a reference to the string slice
// trim() returns a string slice like the previous exercise
// replace() returns a String
// to_lowercase() returns a String

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
