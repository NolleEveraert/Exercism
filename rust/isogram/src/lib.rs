use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let parsed_string: Vec<char> = candidate
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| if c.is_alphabetic() { c.to_ascii_lowercase() } else { c })
        .collect();

    let unique_chars: HashSet<char> = parsed_string.iter().cloned().collect();

    parsed_string.len() == unique_chars.len()
}
