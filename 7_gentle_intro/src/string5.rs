// string5.rs
fn main() {
    let mut s = String::new();
    // initially empty
    s.push('H');
    s.push_str("ello");
    s.push(' ');
    s += "World!"; // short for push_str
    s.pop();

    assert_eq!(s, "Hello World");
}
