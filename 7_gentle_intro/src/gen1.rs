// gen1.rs
// compile error

fn sqr<T>(x: T) -> T {
    x * x
}

fn main() {
    let res = sqr(10.0);
    println!("res {}", res);
}
