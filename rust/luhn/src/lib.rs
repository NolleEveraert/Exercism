/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut digits: Vec<u32> = Vec::new();
    for c in code.chars().filter(|c| !c.is_whitespace()) {
        if let Some(d) = c.to_digit(10) {
            digits.push(d);
        } else {
            return false;
        }
    }

    // make sure that the number is the correct length
    if digits.len() < 2 {
        return false;
    }

    // double every second digit and subtract 9 if too big
    for i in 0..digits.len() {
        if (digits.len() - i) % 2 == 0 {
            digits[i] = digits[i] * 2;
            if digits[i] > 9 {
                digits[i] -= 9;
            }
        }
    }

    // Sum all the digits
    let sum: u32 = digits.iter().sum();

    return sum % 10 == 0;
}
