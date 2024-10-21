use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    let input = input.trim();

    if input.is_empty() {
        println!("Please input a ascii string")
    }

    let mut sum: u32 = 0;
    let mut count: usize = 0;

    for ch in input.chars() {
        let ascii_value = ch as u32;
        sum += ascii_value;
        count += 1;
    }

    let avg: f64 = sum as f64 / count as f64;
    println!("{}", avg)
}
