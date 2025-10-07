/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut check_sum = 0;
    let mut must_double = false;
    let mut digit_count = 0;
    for c in code.chars().rev() {
        match c {
            '0'..='9' => {
                let mut digit = c.to_digit(10).unwrap();
                if must_double {
                    digit *= 2;
                    if digit > 9 { digit -= 9;}
                }
                digit_count += 1;
                check_sum += digit;
                must_double = !must_double;
            },
            _ if c.is_whitespace() => (),
            _ => return false,
        }
    }
    digit_count > 1 && check_sum % 10 == 0
}
