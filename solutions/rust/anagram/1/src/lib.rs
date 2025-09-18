use grapheme::prelude::*;
use std::collections::HashSet;

fn word_to_sorted_chars(word: &str) -> Vec<char> {
    let word = Graphemes::from_usvs(word);
    let mut sorted_chars: Vec<char> = word.chars().collect();
    sorted_chars.sort();
    sorted_chars
}

fn check_anagram(word_chars_sorted: &Vec<char>, candidate: &str) -> bool {
    if candidate.len() == word_chars_sorted.len() {
        word_to_sorted_chars(candidate) == *word_chars_sorted
    } else {
        false
    }
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lc = word.to_lowercase();
    let mut results: HashSet<&str> = HashSet::new();
    let word_chars_sorted: Vec<char> = word_to_sorted_chars(&word_lc);
    for candidate in possible_anagrams {
        let candidate_lc = candidate.to_lowercase();
        if word_lc != candidate_lc && check_anagram(&word_chars_sorted, &candidate_lc) {
            results.insert(&candidate);
        }
    }
    results
}
