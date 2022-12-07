pub fn solve(input: String) {
    let split_input: Vec<&str> = input.split("\r\n\r\n").collect();
    let (stacks, moves) = (split_input[0], split_input[1]);
    let mut matrix = get_matrix(stacks);
    let moves = get_moves(moves);
    for move_ in moves.into_iter() {
        let mut popped_crates: Vec<char> = Vec::new();
        for _ in 0..move_.quantity {
            popped_crates.push(matrix[move_.source - 1].pop().unwrap());
        }
        popped_crates.reverse();
        for popped in popped_crates {
            matrix[move_.dest - 1].push(popped);
        }
    }
    let top_crates = matrix.iter().map(|x| x.last().unwrap()).collect::<String>();
    println!("{top_crates}");
}

fn get_matrix(stacks: &str) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = vec![Vec::new(); 9];
    for line in stacks.lines() {
        let (mut line_index, mut crate_index): (usize, usize) = (0, 0);
        for line_char in line.chars() {
            if line_char == '1' {
                break;
            }
            if line_index > 0 && (line_index - 1) % 4 == 0 {
                if line_char != ' ' {
                    matrix[crate_index].push(line_char);
                }
                crate_index += 1;
            }
            line_index += 1;
        }
    }
    for i in 0..matrix.len() {
        matrix[i].reverse();
    }
    matrix
}

#[derive(Debug)]
struct Move {
    quantity: usize,
    source: usize,
    dest: usize,
}

impl Move {
    pub fn from_str(input: &str) -> Self {
        // move 5 from 3 to 6
        let chunks: Vec<&str> = input.split(' ').collect();
        Move {
            quantity: chunks[1].parse::<usize>().unwrap(),
            source: chunks[3].parse::<usize>().unwrap(),
            dest: chunks[5].parse::<usize>().unwrap(),
        }
    }
}

fn get_moves(moves: &str) -> Vec<Move> {
    let mut result = Vec::new();
    for line in moves.lines() {
        result.push(Move::from_str(line));
    }
    result
}
