pub fn egg_count(display_value: u32) -> usize {
    let mut decimal = display_value;
    let mut sum: usize = 0;
    while decimal != 0 {
        if decimal % 2 != 0 {
            sum += 1;
        }

        decimal /= 2;
    }

    sum
}
