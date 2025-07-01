#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    } else if is_sublist(first_list, second_list) {
        return Comparison::Sublist;
    } else if is_superlist(first_list, second_list) {
        return Comparison::Superlist;
    } else {
        return Comparison::Unequal;
    }
}

pub fn is_sublist(first: &[i32], last: &[i32]) -> bool {
    let sub_length: usize = first.len();

    if sub_length == 0 {
        return true;
    }

    for i in 0..last.len() {
        let mut is_sub: bool = true;
        for j in 0..sub_length {
            if i + j >= last.len() {
                return false;
            }

            if last[i + j] != first[j] {
                is_sub = false;
                break;
            }
        }

        if is_sub {
            return true;
        }
    }

    return false;
}

pub fn is_superlist(first: &[i32], last: &[i32]) -> bool {
    return is_sublist(last, first);
}
