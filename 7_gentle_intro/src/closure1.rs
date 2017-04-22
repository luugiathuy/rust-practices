// closure1.rs
fn main() {
    let f = |x| x * x;

    let res = f(10);

    // let resf = f(1.2); // error! the first call fixes type of argument

    println!("res {}", res);
}
