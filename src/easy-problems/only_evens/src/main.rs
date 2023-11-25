fn noOdds(arr: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for &item in arr {
        if item % 2 == 0 {
            result.push(item);
        }
    }
    result
}

/* fn noOdds(arr: &[i32]) -> Vec<i32> {
    arr.iter().filter_map(|&x| if x % 2 == 0 { Some(x) } else { None }).collect()
}

fn noOdds(arr: &[i32]) -> Vec<i32> {
    arr.iter().filter(|&x| x % 2 == 0).cloned().collect()
} */

fn main() {
    println!("Hello, world!");
}
