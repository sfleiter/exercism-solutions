pub fn reverse(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    for c in input.chars().rev() {
        result.push(c);
    }
    result
}
