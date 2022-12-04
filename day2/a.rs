use std::io;
use std::collections::HashMap;

enum Condition {
    LOSE,
    TIE,
    WIN,
}

const WIN_AMT: u32 = 6;
const LOSE_AMT: u32 = 0;
const TIE_AMT: u32 = 3;

#[derive(PartialEq, Eq)]
enum Hand {
    ROCK,
    PAPER,
    SCISSORS
}

fn get_hand(x:char) -> Hand {
    match x {
        'A' => Hand::ROCK,
        'X' => Hand::ROCK,
        'B' => Hand::PAPER,
        'Y' => Hand::PAPER,
        'C' => Hand::SCISSORS,
        'Z' => Hand::SCISSORS,
        _ => Hand::ROCK
    }
}

fn get_condition(x:char) -> Condition {
    match x {
        'X' => Condition::LOSE,
        'Y' => Condition::TIE,
        'Z' => Condition::WIN,
        _ => Condition::LOSE
    }
}

fn play(a:char, b:char) -> u32 {
    if get_hand(a) == Hand::ROCK {
        match get_hand(b) {
            Hand::PAPER => return WIN_AMT,
            Hand::SCISSORS => return LOSE_AMT,
            Hand::ROCK => return TIE_AMT
        }
    } else if get_hand(a) == Hand::PAPER {
        match get_hand(b) {
            Hand::PAPER => return TIE_AMT,
            Hand::ROCK => return LOSE_AMT,
            Hand::SCISSORS => return WIN_AMT
        }
    } else if get_hand(a) == Hand::SCISSORS {
        match get_hand(b) {
            Hand::SCISSORS => return TIE_AMT,
            Hand::ROCK => return WIN_AMT,
            Hand::PAPER => return LOSE_AMT
        }
    }
    return 0;
}

fn play2(a:char, b:char) -> u32 {
    let score_map = HashMap::from([
        ('X', 1),
        ('Y', 2),
        ('Z', 3),       
    ]);

    if get_hand(a) == Hand::ROCK {
        match get_condition(b) {
            Condition::LOSE => return LOSE_AMT + score_map[&'Z'],
            Condition::TIE => return TIE_AMT + score_map[&'X'],
            Condition::WIN => return WIN_AMT + score_map[&'Y']
        }
    } else if get_hand(a) == Hand::PAPER {
        match get_condition(b) {
            Condition::LOSE => return LOSE_AMT + score_map[&'X'],
            Condition::TIE => return TIE_AMT + score_map[&'Y'],
            Condition::WIN => return WIN_AMT + score_map[&'Z']
        }
    } else if get_hand(a) == Hand::SCISSORS {
        match get_condition(b) {
            Condition::LOSE => return LOSE_AMT + score_map[&'Y'],
            Condition::TIE => return TIE_AMT + score_map[&'Z'],
            Condition::WIN => return WIN_AMT + score_map[&'X']
        }
    }
    return 0;
}

fn main() {

    let score_map = HashMap::from([
        ('A', 1),
        ('B', 2),
        ('C', 3),
        ('X', 1),
        ('Y', 2),
        ('Z', 3),       
    ]);

    let mut score = 0;
    let mut score2 = 0;
    loop {
        let mut line = String::new();
        let bytes = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        if bytes == 0 {
            break;
        }
        let opp: char = line.chars().nth(0).unwrap();
        let you: char = line.chars().nth(2).unwrap();

        score += play(opp, you) + score_map[&you];
        score2 += play2(opp, you);
    }

    println!("Pt1 is {score}");
    println!("Pt2 is {score2}");
}