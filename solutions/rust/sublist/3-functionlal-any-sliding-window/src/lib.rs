use crate::Comparison::{Sublist, Superlist, Unequal};

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    match (a.len(), b.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (m, n) if m > n =>
            if a.windows(b.len()).any(|s| s == b) { Superlist } else { Unequal },
        (m, n) if n > m =>
            if b.windows(a.len()).any(|s| s == a) { Sublist } else { Unequal },
        (_, _) => if a == b { Comparison::Equal } else { Unequal  },
    }
}
