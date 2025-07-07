pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return String::new();
    }
    let mut result: String = String::new();

    for i in 0..list.len() - 1 {
        let proverb = format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]);
        result.push_str(&proverb);
    }

    result.push_str(&format!("And all for the want of a {}.", list[0]));

    result
}
