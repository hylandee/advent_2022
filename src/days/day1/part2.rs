pub fn solve(input: String) {
    let mut sum = 0;
    let mut sums: Vec<i32> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            sums.push(sum);
            sum = 0;
            continue;
        }
        sum += line.parse::<i32>().unwrap();
    }
    let top3 = top_3(sums);
    println!("top3: {:?}", top3);
    let total: i32 = top3.iter().sum();
    println!("thats {} total", total);
}

fn top_3(nums: Vec<i32>) -> Vec<i32> {
    let mut temp = nums.clone();
    temp.sort();
    temp.reverse();
    vec![temp[0], temp[1], temp[2]]
}
