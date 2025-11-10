#![no_main]

use fuzz_binary_search::binary_search;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|input: (i32, Vec<i32>)| {
    let (needle, haystack) = input;
    let result = binary_search(needle, &haystack);
    match result {
        Some(idx) => assert_eq!(haystack[idx], needle),
        None => assert!(!haystack.contains(&needle)),
    }
});
