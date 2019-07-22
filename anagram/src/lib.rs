use std::collections::HashSet;

fn norm(word: &str) -> Vec<char> {
    let mut chars: Vec<_> = word.to_lowercase().chars().collect();
    chars.sort();
    chars
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let norm_word = norm(word);
    possible_anagrams
        .iter()
        .filter(|candidate| candidate.to_lowercase() != word.to_lowercase())
        .filter(|candidate| norm(candidate) == norm_word)
        .cloned()
        .collect()
}
