use super::common::find_distinct_substring;

pub fn solve(input: String) {
    let answer = find_distinct_substring(&input, 14);
    println!("Distinct string found at char {answer}");
}
