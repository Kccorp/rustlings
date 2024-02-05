// modules3.rs
//
// You can use the 'use' keyword to bring module paths from modules from
// anywhere and especially from the Rust standard library into your scope. Bring
// SystemTime and UNIX_EPOCH from the std::time module. Bonus style points if
// you can do it with one line!
//
// Execute `rustlings hint modules3` or use the `hint` watch subcommand for a
// hint.

//WRITE UP: This exercise is asking us to use the 'use' keyword to bring in the SystemTime and UNIX_EPOCH from the
// std::time module. So we can import both of these with one line of code. SystemTime is a struct that represents a
// point in time. UNIX_EPOCH is a constant that represents the start of the UNIX epoch. The UNIX epoch is the time
// 00:00:00 UTC on 1 January 1970. We can use these to get the current time and the time since the UNIX epoch.

// TODO: Complete this use statement
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
