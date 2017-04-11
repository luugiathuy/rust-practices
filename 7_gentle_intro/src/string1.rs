// string1.rs
// &str pronounced 'string slice'
fn dump(s: &str) {
    println!("str '{}'", s);
}

fn main() {
    let text = "hello dolly";
    let s = text.to_string(); // s is type String

    dump(text);
    dump(&s); // the borrow operator can coerce String into &str, just as Vec could be coerced into &[]
}
