
fn is_safe_bridge(bridge: &str) -> bool {
    !bridge.contains(' ')
}

/* fn is_safe_bridge(bridge: &str) -> bool {
    bridge.chars().all(|c| c != ' ')
}
*/

/* fn is_safe_bridge(bridge: &str) -> bool {
    bridge.chars().fold(true, |acc, c| acc && c != ' ')
} */

fn main() {
   is_safe_bridge("## #");
}
