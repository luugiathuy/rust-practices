#[macro_use] extern crate text_io;
extern crate counting_sheep;

use counting_sheep::solve;

fn main() {
    let test_case_count: i32 = read!();
    for i in 0..test_case_count {
        print!("Case #{}: ", i + 1);

        let n: i64 = read!();
        let result = solve(n);
        match result {
            -1 =>  println!("INSOMNIA"),
            _ => println!("{}", result),
        }
    }
}
