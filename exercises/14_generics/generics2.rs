// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.

//WRITE UP: We are to rewrite the Wrapper struct to use generics so that it can store any type. We are to implement the
// Wrapper struct to use generics and then implement the new function to return a Wrapper with the value passed to it.
// We are using <T> to specify that Wrapper is a generic typ and we pass it through the struct and the new function.
// Don't forget to add <T> to impl Wrapper<T> to specify that Wrapper is a generic type and be able to use it in your
// function parameter.

struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper <T>{
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
