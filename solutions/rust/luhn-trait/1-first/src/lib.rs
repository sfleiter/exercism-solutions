use std::iter::from_fn;

pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl Luhn for &str {
    fn valid_luhn(&self) -> bool {
        let mut num: u64 = 0;
        for c in self.chars().filter(|&c| !c.is_whitespace()) {
            match c {
                '0'..='9' => {
                    let c_as_digit: u64 = c.to_digit(10).unwrap() as u64;
                    num = num * 10 + c_as_digit;
                },
                _ => return false,
            }
        }
        num.valid_luhn()
    }
}
impl Luhn for String {
    fn valid_luhn(&self) -> bool {
        (&self.as_str()).valid_luhn()
    }
}

macro_rules! impl_luhn {
    ( $t:ty) => {
        impl Luhn for $t {
            fn valid_luhn(&self) -> bool {
                let num: u64 = *self as u64;
                is_valid_luhn(num)
            }
        }
    }
}

impl_luhn!(u8);
impl_luhn!(u16);
impl_luhn!(u32);
impl_luhn!(u64);
impl_luhn!(usize);

pub(crate) fn is_valid_luhn(code: u64) -> bool {
    let mut check_sum = 0;
    let mut must_double = false;
    let mut digit_count = 0;
    for digit in code.reverse_digits() {
        match digit {
            0..=9 => {
                let mut digit = digit;
                if must_double {
                    digit *= 2;
                    if digit > 9 { digit -= 9;}
                }
                digit_count += 1;
                check_sum += digit;
                must_double = !must_double;
            },
            _ => panic!("Unexpected digit {}", digit),
        }
    }
    digit_count > 1 && check_sum % 10 == 0
}

pub(crate) trait ReverseDigits {
    fn reverse_digits(&self) -> impl Iterator<Item = u8>;
}

impl ReverseDigits for u64 {
    fn reverse_digits(&self) -> impl Iterator<Item = u8> {
        let mut num: u64 = *self;
        from_fn(move || {
            match num {
                0 => None,
                _ => {
                    let result = (num % 10) as u8;
                    num /= 10;
                    Some(result)
                },
            }
        })
    }
}
