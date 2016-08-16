pub fn solve(n: i64) -> i64 {
    let mut result = -1;
    if n != 0 {
        let mut check_digits = [false; 10];
        let mut i = 1;
        loop {
            let count = n * i;
            let count_str = count.to_string();

            let digits = count_str.chars()
                .map(|c| c.to_digit(10).unwrap() as usize);

            for digit in digits {
                check_digits[digit] = true;
            }

            if !check_digits.into_iter().any(|x| !x) {
                result = count;
                break;
            }

            i += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_zero() {
        assert_eq!(-1, solve(0));
    }

    #[test]
    fn test_one() {
        assert_eq!(10, solve(1));
    }

    #[test]
    fn test_two() {
        assert_eq!(90, solve(2));
    }

    #[test]
    fn test_11() {
        assert_eq!(110, solve(11));
    }

    #[test]
    fn test_1692() {
        assert_eq!(5076, solve(1692));
    }
}
