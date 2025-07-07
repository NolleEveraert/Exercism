use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut converted_tree: BTreeMap<char, i32> = BTreeMap::new();

    for (value, chars) in h {
        for char in chars {
            converted_tree.insert(char.to_ascii_lowercase(), *value);
        }
    }

    return converted_tree;
}
