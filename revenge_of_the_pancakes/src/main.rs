#[macro_use] extern crate text_io;
extern crate revenge_of_the_pancakes;

use revenge_of_the_pancakes::solve;

fn main() {
    let test_case_count: i32 = read!();
    for i in 0..test_case_count {
        let pancakes_string: String = read!();
        let result = solve(&pancakes_string);
        println!("Case #{}: {}", i + 1, result);
    }
}
