use std::vec::Vec;

pub fn binary_search(needle: i32, haystack: Vec<i32>) -> Option<usize> {
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

use proptest::prelude::*;

#[warn(dead_code)]
fn needle_and_haystack() -> impl Strategy<Value = (i32, Vec<i32>)> {
    (
        prop::num::i32::ANY,
        proptest::collection::vec(prop::num::i32::ANY, 0..100),
    )
}

proptest! {
    #[test]
    fn proptest_binary_search((needle, mut haystack) in needle_and_haystack()) {
        haystack.sort_unstable();
        let result = binary_search(needle, haystack.clone());

        match result {
            Some(index) => {
                prop_assert_eq!(haystack[index], needle);
            },
            None => {
                prop_assert!(!haystack.contains(&needle));
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
