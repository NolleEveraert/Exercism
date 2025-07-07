pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        panic!("Length cannot be zero");
    }

    digits
        .chars()
        .collect::<Vec<_>>()
        .windows(len)
        .map(|window| window.iter().collect())
        .collect()
}
