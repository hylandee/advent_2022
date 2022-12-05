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
    let (opp, outcome) = (chars[0], chars[1]);
    let player = match (opp, outcome) {
        ('A', 'X') | ('B', 'Z') | ('C', 'Y') => 'S',
        ('A', 'Y') | ('B', 'X') | ('C', 'Z') => 'R',
        (_, _) => 'P',
    };
    symbol_points(player) + battle_points((opp, player))
}

fn symbol_points(sym: char) -> i32 {
    match sym {
        'R' => 1,
        'P' => 2,
        _ => 3,
    }
}

fn battle_points(game: (char, char)) -> i32 {
    match game {
        ('A', 'R') | ('B', 'P') | ('C', 'S') => 3,
        ('A', 'S') | ('B', 'R') | ('C', 'P') => 0,
        (_, _) => 6,
    }
}
