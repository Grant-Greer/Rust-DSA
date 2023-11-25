fn equal(a: i32, b: i32, c: i32) -> u8 {
    if a == b && b == c {
        3
    } else if a == b || b == c || a == c {
        2
    } else {
        0
    }
}

fn main() {
    println!("Hello, world!");
}
