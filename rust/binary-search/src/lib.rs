use std::cmp::Ordering;

pub fn find<T, U>(array: T, key: U) -> Option<usize>
where
    T: AsRef<[U]>,
    U: Ord,
{
    let array = array.as_ref();
    if array.is_empty() {
        return None;
    }
    let mut left = 0;
    let mut right = array.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;
        match array[mid].cmp(&key) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => left = mid + 1,
            Ordering::Greater => {
                if mid == 0 {
                    return None;
                }
                right = mid - 1;
            }
        }
    }

    None
}
