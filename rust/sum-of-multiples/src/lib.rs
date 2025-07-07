use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut common_factors: HashSet<u32> = HashSet::new();

    for factor in factors {
        if *factor != 0 {
            for i in (*factor..limit).step_by(*factor as usize) {
                if limit % i == 0 {}
                common_factors.insert(i);
            }
        }
    }

    common_factors.iter().sum()
}
