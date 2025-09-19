use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

fn to_sorted_graphemes(word: &str) -> Vec<&str> {
    let mut graphemes = word.graphemes(true).collect::<Vec<&str>>();
    graphemes.sort();
    graphemes
}

fn check_anagram(word_graphemes_sorted: &Vec<&str>, candidate: &str) -> bool {
    to_sorted_graphemes(candidate) == *word_graphemes_sorted
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lc = word.to_lowercase();
    let mut results: HashSet<&str> = HashSet::new();
    let word_graphemes_sorted: Vec<&str> = to_sorted_graphemes(&word_lc);
    for candidate in possible_anagrams {
        let candidate_lc = candidate.to_lowercase();
        if word_lc != candidate_lc && check_anagram(&word_graphemes_sorted, &candidate_lc) {
            results.insert(candidate);
        }
    }
    results
}
