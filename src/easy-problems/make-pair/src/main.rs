#[derive(PartialEq, Debug)]
struct Pair(i32, i32);

fn make_pair(first: i32, second: i32) -> Pair {
    Pair(first, second)
}

fn main() {
    make_pair(56, 93);
}

#[cfg(test)]

mod tests {
    use super::*;
    
    #[test]
    fn test1() {
        assert_eq!(make_pair(1, 2), Pair(1, 2));
        assert_eq!(make_pair(21, 82), Pair(21, 82));
        assert_eq!(make_pair(4213, 526), Pair(4213, 526));
    }

}
