// fun3.rs

fn abs(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}

fn clamp(x: f64, low: f64, high: f64) -> f64 {
    if x < low {
        low
    } else if high < x {
        high
    } else {
        x
    }
}

fn fact(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * fact(n - 1)
    }
}

fn main() {
    let res1 = abs(-10.0);
    let res2 = clamp(1.5, 0.0, 1.0);
    let res3 = fact(4);
    println!("res1 is {}", res1);
    println!("res2 is {}", res2);
    println!("res3 is {}", res3);
}
