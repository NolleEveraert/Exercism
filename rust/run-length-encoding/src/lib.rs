pub fn encode(source: &str) -> String {
    if source.is_empty() {
        return String::new();
    }

    let mut encoded_string = String::new();
    let mut chars = source.chars();
    let mut current_char = chars.next().unwrap();
    let mut count = 1;

    for char in chars {
        if char == current_char {
            count += 1;
        } else {
            if count > 1 {
                encoded_string.push_str(&count.to_string());
            }
            encoded_string.push(current_char);
            current_char = char;
            count = 1;
        }
    }

    if count > 1 {
        encoded_string.push_str(&count.to_string());
    }
    encoded_string.push(current_char);

    encoded_string
}

pub fn decode(source: &str) -> String {
    if source.is_empty() {
        return String::new();
    }

    let mut decoded_string: String = String::new();
    let chars = source.chars();
    let mut quantity: String = String::new();

    for char in chars {
        if char.is_numeric() {
            quantity.push(char);
        } else {
            let amount: u16;
            if !quantity.is_empty() {
                amount = quantity.parse().unwrap();
            } else {
                amount = 1;
            }

            for _ in 0..amount {
                decoded_string.push(char);
            }
            quantity = String::new();
        }
    }

    decoded_string
}
