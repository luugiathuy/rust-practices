pub fn solve(s: &str) -> u32 {
    let mut count: u32 = 0;

    let mut pancakes: Vec<char> = s.chars().collect();

    for i in (0..pancakes.len()).rev() {
        if pancakes[i] == '-' {
            // Flip top '+'s (if any) to '-'s so that
            // it will become '+'s when we flip from 0 to i
            let mut j = 0;
            while pancakes[j] == '+' { j += 1; }
            if j > 0 {
                for p in &mut pancakes[0..j] { *p = '-'; }
                count += 1;
            }

            // Flip from 0 to i
            pancakes[0..i+1].reverse();
            for p in &mut pancakes[0..i+1] { *p = flip(*p); }
            count += 1;
        }
    }

    count
}

fn flip(c: char) -> char {
    if c == '-' { '+' } else { '-' }
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_one_happy() {
        assert_eq!(0, solve("+"));
    }

    #[test]
    fn test_one_sad() {
        assert_eq!(1, solve("-"));
    }

    #[test]
    fn test_all_happy() {
        assert_eq!(0, solve("++++"));
    }

    #[test]
    fn test_all_sad() {
        assert_eq!(1, solve("----"));
    }

    #[test]
    fn test_happy_sad() {
        assert_eq!(2, solve("+-"));
    }

    #[test]
    fn test_sad_happy() {
        assert_eq!(1, solve("-+"));
    }

    #[test]
    fn test_sad_sad_happy_sad() {
        assert_eq!(3, solve("--+-"));
    }
}
