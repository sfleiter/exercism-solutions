use crate::Comparison::{Sublist, Superlist};

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if let Some(value) = check_one_list_empty(first_list, second_list) {
        return value;
    }
    if first_list.len() > second_list.len() {
        check_sublist_by_window(first_list, second_list, Superlist)
    } else {
        check_sublist_by_window(second_list, first_list, Sublist)
    }
}

fn check_one_list_empty(first_list: &[i32], second_list: &[i32]) -> Option<Comparison> {
    match (first_list.is_empty(), second_list.is_empty()) {
        (true, true) => Some(Comparison::Equal),
        (true, false) => Some(Sublist),
        (false, true) => Some(Superlist),
        (false, false) => None,
    }
}

fn check_sublist_by_window(first_list: &[i32], second_list: &[i32], comp: Comparison) -> Comparison {
    for first_slice in first_list.windows(second_list.len()) {
        if first_slice == second_list {
            return if first_list.len() == second_list.len() {
                Comparison::Equal
            } else {
                comp
            }
        }
    }
    Comparison::Unequal
}
