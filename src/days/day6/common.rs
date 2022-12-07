use std::collections::HashSet;

pub fn find_distinct_substring(input: &str, size: usize) -> usize {
    let (mut left, mut right): (usize, usize) = (0, size);
    loop {
        let slize = &input[left..right];
        if are_chars_distinct(slize) {
            return right;
        }
        left += 1;
        right += 1;
    }
}

pub fn are_chars_distinct(sample: &str) -> bool {
    let mut chars = HashSet::new();
    for element in sample.chars() {
        if chars.contains(&element) {
            return false;
        }
        chars.insert(element);
    }
    true
}
