// A - Rock, B - Paper, C - Scissors
// X - ROck, Y - Paper, Z - Scissors

// Opponent
const OP_ROCK: char = 'A';
const OP_PAPER: char = 'B';
const OP_SCISSORS: char = 'C';

// Player
const PL_ROCK: char = 'X';
const PL_PAPER: char = 'Y';
const PL_SCISSORS: char = 'Z';

fn main() {
    let input = include_str!("../input.txt");

    let mut score = 0;
    for var in input.lines() {
        let opponent = var.chars().nth(0).unwrap();
        let player = var.chars().nth(2).unwrap();

        score += calc_score(opponent, player);
    }

    println!("{}", score);
}

fn calc_score(opponent: char, player: char) -> i32 {
    let mut output: i32 = match player {
        PL_ROCK => 1,
        PL_PAPER => 2,
        PL_SCISSORS => 3,
        _ => 0
    };

    if player == PL_ROCK {
        match opponent {
            OP_ROCK => output += 3,
            OP_PAPER => output += 0,
            OP_SCISSORS => output += 6,
            _ => output += 0
        }
    } else if player == PL_PAPER {
        match opponent {
            OP_ROCK => output += 6,
            OP_PAPER => output += 3,
            OP_SCISSORS => output += 0,
            _ => output += 0
        }
    } else if player == PL_SCISSORS {
        match opponent {
            OP_ROCK => output += 0,
            OP_PAPER => output += 6,
            OP_SCISSORS => output += 3,
            _ => output += 0
        }
    }
    return output;
}
