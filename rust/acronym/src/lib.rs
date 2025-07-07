pub fn abbreviate(phrase: &str) -> String {
    let mut result = String::new();

    // Split on whitespace and punctuation
    for word in phrase.split(|c: char| c.is_whitespace() || (c.is_ascii_punctuation() && c != '\'')) {
        if word.is_empty() {
            continue;
        }

        // Clean the word by removing any remaining punctuation except letters
        let clean_word: String = word.chars().filter(|c| c.is_alphabetic()).collect();
        if clean_word.is_empty() {
            continue;
        }

        // Add the first character of the clean word
        if let Some(first_char) = clean_word.chars().next() {
            result.push(first_char.to_ascii_uppercase());
        }

        // Handle camelCase: look for uppercase letters after lowercase letters
        let mut prev_was_lower = false;
        for ch in clean_word.chars().skip(1) {
            if ch.is_uppercase() && prev_was_lower {
                result.push(ch);
            }
            prev_was_lower = ch.is_lowercase();
        }
    }

    result
}
