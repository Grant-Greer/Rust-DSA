fn maskify(mut s: String) -> String {
    let len = s.len();
    if len > 4 {
        s.replace_range(0..len - 4, &"#".repeat(len - 4));
    }
    s
}

fn main() {
    println!("{}", maskify("4556364607935616".to_string())); // ➞ "############5616"
    println!("{}", maskify("64607935616".to_string()));      // ➞ "#######5616"
    println!("{}", maskify("1".to_string()));                 // ➞ "1"
    println!("{}", maskify("".to_string()));                  // ➞ ""
}