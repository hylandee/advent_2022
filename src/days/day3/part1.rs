pub fn solve(input: String) {
    let mut sum = 0;
    for line in input.lines() {
        let len = line.len();
        let pair: (&str, &str) = (&line[0..(len / 2)], &line[(len / 2)..len]);
        sum += get_priority(pair);
    }
    println!("sum: {sum}");
}

fn get_priority(pair: (&str, &str)) -> u32 {
    for i in pair.0.chars() {
        for j in pair.1.chars() {
            if i == j {
                return match i as u32 {
                    x if x <= 90 => x - 38,
                    x => x - 96,
                };
            }
        }
    }
    return 0;
}
