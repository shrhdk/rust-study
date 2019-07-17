use std::collections::HashSet;

fn normalize(word: &str) -> Vec<char> {
    let mut chars: Vec<char> = word.to_lowercase().chars().collect();
    chars.sort_by(Ord::cmp);
    chars
}

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let w = normalize(word);
    let mut result: HashSet<&'a str> = HashSet::new();
    for candidate in possible_anagrams {
        let c = normalize(candidate);
        if c == w && word.to_lowercase() != candidate.to_lowercase() {
            result.insert(*candidate);
        }
    }
    result
}
