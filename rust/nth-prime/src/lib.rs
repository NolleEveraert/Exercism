pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let mut primes = vec![2];
    let mut candidate = 3;

    while primes.len() <= n as usize {
        let sqrt_candidate = (candidate as f64).sqrt() as u32;
        let is_prime = primes
            .iter()
            .take_while(|&&p| p <= sqrt_candidate)
            .all(|&p| candidate % p != 0);

        if is_prime {
            primes.push(candidate);
        }
        candidate += 2; // Skip even numbers
    }

    primes[n as usize]
}
