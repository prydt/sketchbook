use std::vec::Vec;

pub fn binary_search(needle: i32, haystack: &Vec<i32>) -> Option<usize> {
    if haystack.is_empty() {
        return None;
    }
    let mut low = 0;
    let mut high = haystack.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let mid_val = haystack[mid];

        if mid_val < needle {
            low = mid + 1;
        } else if mid_val > needle {
            high = mid - 1;
        } else {
            return Some(mid);
        }
    }

    return None;
}

