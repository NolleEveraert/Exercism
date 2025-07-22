pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| translate_word(word))
        .collect::<Vec<String>>()
        .join(" ")
}

fn translate_word(input: &str) -> String {
    // Exclude empty strings
    if input.len() == 0 {
        return String::new();
    }

    // Rule 1: Vowels, "xr", and "yt"
    if input.starts_with('a')
        || input.starts_with('e')
        || input.starts_with('i')
        || input.starts_with('o')
        || input.starts_with('u')
        || input.starts_with("xr")
        || input.starts_with("yt")
    {
        return format!("{}ay", input);
    }

    // Find the first vowel position
    let mut consonant_end = 0;
    let chars: Vec<char> = input.chars().collect();

    // Rule 3: Handle "qu" combinations
    for i in 0..chars.len() {
        if i + 1 < chars.len() && chars[i] == 'q' && chars[i + 1] == 'u' {
            consonant_end = i + 2; // Include both 'q' and 'u'
            break;
        }

        // Rule 4: Handle consonant(s) + "y"
        if chars[i] == 'y' && i > 0 {
            consonant_end = i;
            break;
        }

        // Rule 2: Find first vowel
        if "aeiou".contains(chars[i]) {
            consonant_end = i;
            break;
        }

        // If we reach the end without finding a vowel, move all consonants
        if i == chars.len() - 1 {
            consonant_end = chars.len();
        }
    }

    // Move consonants to end and add "ay"
    let consonants = &input[..consonant_end];
    let rest = &input[consonant_end..];
    format!("{}{}ay", rest, consonants)
}
