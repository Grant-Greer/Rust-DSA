
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    // Remove the trailing newline character(s), but keep spaces
    input = input.trim_end_matches(&['\r', '\n'][..]).to_string();

    let mut sum: u32 = 0;
    let mut count: usize = 0;

    for ch in input.chars() {
        let ascii_value = ch as u32;
        sum += ascii_value;
        count += 1;
    }

    let avg_char = if count == 0 {
        ' '
    } else {
        let avg: f64 = sum as f64 / count as f64;
        let floor_avg = avg.floor() as u32;
        char::from_u32(floor_avg).unwrap_or(' ')
    };

    println!("{}", avg_char);
}

