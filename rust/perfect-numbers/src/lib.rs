#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    let mut factors: Vec<u64> = Vec::new();

    for i in 1..=num.isqrt() {
        if num % i == 0 {
            factors.push(i);

            let other = num / i;
            if other != i && other != num {
                factors.push(other);
            }
        }
    }

    // Prime numbers
    if factors.len() <= 1 {
        return Some(Classification::Deficient);
    }

    let sum: u64 = factors.iter().sum();

    print!("{}\n", sum);
    print!("{:?}", factors);

    if sum == num {
        Some(Classification::Perfect)
    } else if sum < num {
        Some(Classification::Deficient)
    } else {
        Some(Classification::Abundant)
    }
}
