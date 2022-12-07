pub fn solve(input: String) {
    let mut sum = 0;
    for line in input.lines() {
        let mut split = line.split(',');
        let (left, right) = (split.next().unwrap(), split.next().unwrap());
        if are_either_contained(left, right) {
            sum += 1;
        }
    }
    println!("sum of complete overlappers: {sum}");
}

fn are_either_contained(left: &str, right: &str) -> bool {
    left_in_right(left, right) || left_in_right(right, left)
}

fn left_in_right(left: &str, right: &str) -> bool {
    let (left, right) = (get_min_max(left), get_min_max(right));
    left.0 >= right.0 && left.1 <= right.1
}

fn get_min_max(input: &str) -> (i32, i32) {
    let mut split = input.split('-');
    (
        split.next().unwrap().parse::<i32>().unwrap(),
        split.next().unwrap().parse::<i32>().unwrap(),
    )
}
