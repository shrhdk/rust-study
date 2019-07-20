pub fn number(user_number: &str) -> Option<String> {
    let mut digits: Vec<_> = user_number.chars().filter(|c| c.is_digit(10)).collect();

    if digits.len() == 11 && digits[0] == '1' {
        digits.remove(0);
    }

    if digits.len() != 10 {
        return None;
    }

    // verify head of area code
    if digits[0] == '0' || digits[0] == '1' {
        return None;
    }

    // verify head of exchange code
    if digits[3] == '0' || digits[3] == '1' {
        return None;
    }

    Some(digits.iter().collect())
}
