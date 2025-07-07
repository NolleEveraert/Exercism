pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut number: u64 = n;
    let mut iterations: u64 = 0;

    while number != 1 {
        if number % 2 == 0 {
            number /= 2;
        } else {
            number = 3 * number + 1;
        }

        iterations += 1;
    }

    Some(iterations)
}
