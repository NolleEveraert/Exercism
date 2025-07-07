pub fn factors(n: u64) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();

    let mut digit: u64 = n;

    while digit % 2 == 0 {
        result.push(2);
        digit /= 2;
    }

    for i in (3..digit.isqrt() + 1).step_by(2) {
        while digit % i == 0 {
            result.push(i);
            digit /= i;
        }
    }

    if digit > 1 {
        result.push(digit);
    }

    result
}
