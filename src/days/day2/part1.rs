pub fn solve(input: String) {
    let mut score = 0;
    for line in input.lines() {
        let chars: Vec<char> = line
            .split(' ')
            .map(str::trim)
            .map(|x| x.chars().next().unwrap())
            .collect();
        score += get_score(chars);
    }
    println!("{score}");
}

fn get_score(chars: Vec<char>) -> i32 {
    println!("{:?}", chars);
    let (opp, player) = (chars[0], chars[1]);
    dbg!(symbol_points(player)) + dbg!(battle_points((opp, player)))
}

fn symbol_points(sym: char) -> i32 {
    match sym {
        'X' => 1,
        'Y' => 2,
        _ => 3,
    }
}

fn battle_points(game: (char, char)) -> i32 {
    match game {
        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
        ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
        (_, _) => 6,
    }
}
