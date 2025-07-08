/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.chars().any(|c| c.is_alphabetic() && c != 'X') {
        return false;
    }

    let char_digits: Vec<char> = isbn
        .chars()
        .filter(|d| d.is_numeric() || (*d == 'X' && isbn.chars().last() == Some(*d)))
        .collect();

    if char_digits.len() != 10 {
        return false;
    }

    let digits: Vec<u16> = char_digits
        .iter()
        .map(|c| {
            if *c == 'X' {
                10
            } else {
                c.to_digit(10).unwrap_or(0) as u16
            }
        })
        .collect();

    let mut sum: u16 = 0;

    for i in 0..10 {
        sum += digits[i] * (10 - i) as u16;
    }

    return sum % 11 == 0;
}
