use std::collections::HashSet;

fn norm(word: &str) -> Vec<char> {
    let mut chars: Vec<_> = word.to_lowercase().chars().collect();
    chars.sort_by(Ord::cmp);
    chars
}

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let norm_word = norm(word);
    possible_anagrams.iter()
        .filter(|candidate| word.to_lowercase() != candidate.to_lowercase())
        .filter(|candidate| norm(candidate) == norm_word)
        .cloned()
        .collect()
}
