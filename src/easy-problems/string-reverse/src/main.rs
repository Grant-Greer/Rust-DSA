#[cfg(test)]
mod tests;

fn reverse (s: String) -> String {
    let mut result = String::new();
    for c in s.chars().rev() {
        result.push(c);
    }
    result
}

/* fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

fn reverse(s: &str) -> String {
    s.chars().fold(String::new(), |mut acc, c| {
        acc.insert(0, c);
        acc
    })
} */




fn main() {
   reverse(String::from("hello"));
}
