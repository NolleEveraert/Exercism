use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    min: u64,
    max: u64,
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        let mut factors: HashSet<(u64, u64)> = HashSet::new();

        for i in self.min..self.max {
            let factor = self.value / i;

            if self.value % i == 0 && i >= self.min && i <= self.max && factor >= self.min && factor <= self.max {
                if i < factor {
                    factors.insert((i, factor));
                } else {
                    factors.insert((factor, i));
                }
            }
        }

        factors
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    fn is_palindrome(n: u64) -> bool {
        let s = n.to_string();
        s == s.chars().rev().collect::<String>()
    }

    let mut smallest: Option<Palindrome> = None;
    let mut highest: Option<Palindrome> = None;

    for product in (min..=max).flat_map(|i| (i..=max).map(move |j| i * j)) {
        if is_palindrome(product) {
            let palindrome = Palindrome {
                value: product,
                min: min,
                max: max,
            };

            if smallest.as_ref().map_or(true, |s| s.value > product) {
                smallest = Some(palindrome.clone());
            }
            if highest.as_ref().map_or(true, |h| h.value < product) {
                highest = Some(palindrome);
            }
        }
    }

    Some((smallest?, highest?))
}
