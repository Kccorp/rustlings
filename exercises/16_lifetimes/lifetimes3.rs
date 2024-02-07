// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

//WRITE UP: need to add a lifetime annotation to the Book struct to ensure that the references in the struct live as
// long as the struct itself. This is done by adding a lifetime annotation to the struct definition, and then using that
// lifetime annotation in the fields of the struct. It requires because the struct is holding references to strings, and
// we need to ensure that the references live as long as the struct itself.

struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
