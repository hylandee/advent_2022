pub fn solve(input: String) {
    let (mut sum, mut max) = (0, 0);
    for line in input.lines() {
        if line.is_empty() {
            max = get_max(max, sum);
            sum = 0;
            continue;
        }
        sum += line.parse::<i32>().unwrap();
    }
    max = get_max(max, sum);
    println!("Final max: {}", max);
}

fn get_max(left: i32, right: i32) -> i32 {
    if right > left {
        right
    } else {
        left
    }
}
