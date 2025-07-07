const DIGITS: [&str; 10] = [
    "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
];

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut bottles_remaining: u32 = start_bottles;
    let mut song: String = String::new();

    for i in 0..take_down {
        let current_bottles = DIGITS[(bottles_remaining - 1) as usize];
        let next_bottles = if bottles_remaining > 1 {
            DIGITS[(bottles_remaining - 2) as usize].to_lowercase()
        } else {
            "no".to_string()
        };

        let current_bottle_word = if bottles_remaining == 1 { "bottle" } else { "bottles" };

        let next_bottle_word = if bottles_remaining - 1 == 1 {
            "bottle"
        } else {
            "bottles"
        };

        let verse = format!(
            "{} green {} hanging on the wall,\n{} green {} hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be {} green {} hanging on the wall.\n",
            current_bottles, current_bottle_word, current_bottles, current_bottle_word, next_bottles, next_bottle_word
        );

        song.push_str(&verse);

        // Add empty line between verses (except for the last one)
        if i < take_down - 1 {
            song.push('\n');
        }

        bottles_remaining -= 1;
    }

    song
}
