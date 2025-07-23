pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound < 2 {
        return Vec::new();
    }
    let mut markings: Vec<bool> = vec![false; (upper_bound + 1) as usize];
    markings[0] = true;
    markings[1] = true;

    for i in 2..=upper_bound {
        if markings[i as usize] {
            continue;
        } else {
            let mut product = 2 * i;
            while product <= upper_bound {
                markings[product as usize] = true;
                product += i;
            }
        }
    }

    markings
        .iter()
        .enumerate()
        .filter(|(_, marked)| !*marked)
        .map(|(index, _)| index as u64)
        .collect()
}
