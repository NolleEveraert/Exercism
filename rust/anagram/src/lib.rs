use std::collections::{HashMap, HashSet};

pub fn get_char_count<'a>(word: &str) -> HashMap<char, i32> {
    let mut char_counts: HashMap<char, i32> = HashMap::new();

    for c in word.to_lowercase().chars() {
        *char_counts.entry(c).or_insert(0) += 1;
    }

    return char_counts;
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let char_count_word: HashMap<char, i32> = get_char_count(&word);
    let mut results: HashSet<&'a str> = HashSet::new();

    for possible_word in possible_anagrams {
        let char_count_possible = get_char_count(&possible_word);
        if char_count_word == char_count_possible
            && word.to_lowercase() != *possible_word.to_lowercase()
        {
            results.insert(&possible_word);
        }
    }

    return results;
}
