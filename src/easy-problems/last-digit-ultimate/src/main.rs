
fn last_dig(a: i32, b: i32, c: i32) -> bool {
    (a * b) % 10 == c % 10
}


fn main() {
   last_dig(5, 1, 125);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn true_tests() {
        assert_eq!(last_dig(1, 1, 1), true);
        assert_eq!(last_dig(12, 15, 10), true);
        assert_eq!(last_dig(15228, 9209, 72162), true);
    }

    #[test]
    fn false_tests() {
        assert_eq!(last_dig(15, 1, 1), false);
        assert_eq!(last_dig(123, 15, 10), false);
        assert_eq!(last_dig(5213, 99219, 6165), false);
        assert_eq!(last_dig(1523, 513, 512), false);
    }

    #[test]
    fn negative_numbers_tests() {
        assert_eq!(last_dig(-15, 1, 1), false);
        assert_eq!(last_dig(123, -15, 10), false);
        assert_eq!(last_dig(-12, 15, -10), true);
        assert_eq!(last_dig(15228, -9209, -72162), true);
    }
}