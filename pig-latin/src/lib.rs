use regex::Regex;

fn translate_word(word: &str) -> String {
    // Rule 1-1
    let re = Regex::new(r"^(xr|yt)\S+$").unwrap();
    if re.is_match(word) {
        return re.replace(word, "${0}ay").to_string();
    }

    // Rule 4
    let re = Regex::new(r"^([bcdfghjklmnpqrstvwxyz]+)(y\S*)$").unwrap();
    if re.is_match(word) {
        return re.replace(word, "${2}${1}ay").to_string();
    }

    // Rule 3
    let re = Regex::new(r"^([bcdfghjklmnpqrstvwxyz]*qu)(\S*)$").unwrap();
    if re.is_match(word) {
        return re.replace(word, "${2}${1}ay").to_string();
    }

    // Rule 2
    let re = Regex::new(r"^([bcdfghjklmnpqrstvwxyz]+)(\S*)$").unwrap();
    if re.is_match(word) {
        return re.replace(word, "${2}${1}ay").to_string();
    }

    // Rule 1-2
    let re = Regex::new(r"^[aeiouy]+\S+$").unwrap();
    if re.is_match(word) {
        return re.replace(word, "${0}ay").to_string();
    }

    word.to_string()
}

pub fn translate(input: &str) -> String {
    input.split_whitespace()
        .map(translate_word)
        .fold(String::new(), |out, word| {
            if out.is_empty() {
                out + &word
            } else {
                out + " " + &word
            }
        })
}
