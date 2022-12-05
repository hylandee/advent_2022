pub fn solve(input: String) {
    let mut sum = 0;
    let mut thrup = ("", "", "");
    let mut thrups: Vec<(&str, &str, &str)> = Vec::new();
    let mut count = 0;
    for line in input.lines() {
        match count {
            0 => {
                thrup.0 = line;
                count += 1;
            }
            1 => {
                thrup.1 = line;
                count += 1;
            }
            2 => {
                thrup.2 = line;
                thrups.push(thrup);
                count = 0;
                continue;
            }
            _ => (),
        }
    }
    for t in thrups {
        sum += get_priority(t);
    }
    println!("sum: {sum}");
}

fn get_priority(thrup: (&str, &str, &str)) -> u32 {
    for i in thrup.0.chars() {
        for j in thrup.1.chars() {
            for k in thrup.2.chars() {
                if i == j && j == k {
                    return match i as u32 {
                        x if x <= 90 => x - 38,
                        x => x - 96,
                    };
                }
            }
        }
    }
    return 0;
}
