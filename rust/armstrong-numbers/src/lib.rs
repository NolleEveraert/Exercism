pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = num.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    print!("{:?}", digits);
    let mut sum: u32 = 0;
    let length: u32 = digits.len() as u32;
    for digit in digits {
        sum += digit.pow(length);
    }

    return num == sum;
}
