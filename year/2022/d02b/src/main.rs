// A - Rock, B - Paper, C - Scissors
// X - ROck, Y - Paper, Z - Scissors

// Opponent
const OP_ROCK: char = 'A';
const OP_PAPER: char = 'B';
const OP_SCISSORS: char = 'C';

// Player
const PL_LOSE: char = 'X';
const PL_DRAW: char = 'Y';
const PL_WIN: char = 'Z';

// Score
const SCORE_ROCK: i32 = 1;
const SCORE_PAPER: i32 = 2;
const SCORE_SCISSORS: i32 = 3;

const ROUND_LOST: i32 = 0;
const ROUND_DRAW: i32 = 3;
const ROUND_WIN: i32 = 6;

fn main() {
    let input = include_str!("../input.txt");

    let mut score = 0;
    for var in input.lines() {
        let opponent = var.chars().nth(0).unwrap();
        let strategy = var.chars().nth(2).unwrap();

        score += calc_score(opponent, strategy);
    }

    println!("{}", score);
}

fn calc_score(opponent: char, strategy: char) -> i32 {
    let mut output: i32 = 0;

    if strategy == PL_LOSE {
        output += ROUND_LOST;
        match opponent {
            OP_ROCK => output += SCORE_SCISSORS,
            OP_PAPER => output += SCORE_ROCK,
            OP_SCISSORS => output += SCORE_PAPER,
            _ => output += 0
        }
    } else if strategy == PL_DRAW {
        output += ROUND_DRAW;
        match opponent {
            OP_ROCK => output += SCORE_ROCK,
            OP_PAPER => output += SCORE_PAPER,
            OP_SCISSORS => output += SCORE_SCISSORS,
            _ => output += 0
        }
    } else if strategy == PL_WIN {
        output += ROUND_WIN;
        match opponent {
            OP_ROCK => output += SCORE_PAPER,
            OP_PAPER => output += SCORE_SCISSORS,
            OP_SCISSORS => output += SCORE_ROCK,
            _ => output += 0
        }
    }
    return output;
}

