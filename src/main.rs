use ::phf::{phf_map, Map};
use inline_colorization::*;
use lazy_static::lazy_static;
use std::{collections::HashMap, fmt::Display, fs};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Copy, Clone, Eq, PartialEq, Hash, EnumIter)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Display for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Choice::Rock => "Rock",
            Choice::Paper => "Paper",
            Choice::Scissors => "Scissors",
        })?;
        Ok(())
    }
}

enum MatchResult {
    Win,
    Tie,
    Lose,
}

// static map variant that works with enums - https://users.rust-lang.org/t/is-there-a-way-to-create-a-constant-map-in-rust/8358/10
lazy_static! {
    static ref BEATS_MAP: HashMap<Choice, Choice> = [
        (Choice::Rock, Choice::Scissors),
        (Choice::Paper, Choice::Rock),
        (Choice::Scissors, Choice::Paper),
    ]
    .iter()
    .copied()
    .collect();
}

// phf variant (doesnt work for enums): https://users.rust-lang.org/t/is-there-a-way-to-create-a-constant-map-in-rust/8358/11
static OPPONENT_CHOICE_MAP: Map<&str, Choice> = phf_map! {
    "A" => Choice::Rock,
    "B" => Choice::Paper,
    "C" => Choice::Scissors,
};
static MY_CHOICE_MAP: Map<&str, Choice> = phf_map! {
    "X" => Choice::Rock,
    "Y" => Choice::Paper,
    "Z" => Choice::Scissors,
};

fn match_result(opponent: Choice, mine: Choice) -> MatchResult {
    let opponent_beats = BEATS_MAP.get(&opponent).expect("Exhaustive BEATS_MAP");
    let mine_beats = BEATS_MAP.get(&mine).expect("Exhaustive BEATS_MAP");
    println!(
        "opponent {} {color_green} beats {} {color_yellow}, mine {} beats {} {color_red}",
        opponent, opponent_beats, mine, mine_beats
    );
    if opponent_beats == &mine {
        MatchResult::Lose
    } else if mine_beats == &opponent {
        MatchResult::Win
    } else {
        MatchResult::Tie
    }
}

fn calc_score(opponent: Choice, mine: Choice) -> u16 {
    let selecion_score = match mine {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };

    let match_score = match match_result(opponent, mine) {
        MatchResult::Win => 6,
        MatchResult::Tie => 3,
        MatchResult::Lose => 0,
    };

    println!(
        "Score: sel={} , match={} => {},",
        selecion_score,
        match_score,
        selecion_score + match_score
    );
    selecion_score + match_score
}

fn process_game(
    content: String,
    perform_logic: for<'a> fn(&'a str, &'a str) -> (Choice, Choice),
) -> String {
    // println!("Input: {}", content);
    let mut total_score: u16 = 0;
    for line in content.lines() {
        let line_split = line.split_whitespace().collect::<Vec<_>>();
        let [enc_opponent, enc_mine] = line_split.as_slice() else {
            panic!("Invalid input line: {}", line)
        };
        let (choice_opp, choice_mine) = perform_logic(enc_opponent, enc_mine);
        let score = calc_score(choice_opp, choice_mine);
        total_score += score;
        println!(
            "Opp: {} {color_cyan}, Me: {} => {} {color_white} ==> TOTAL={}",
            enc_opponent, enc_mine, score, total_score
        );
    }
    total_score.to_string()
}

fn process_part1(content: String) -> String {
    process_game(content, |enc_opponent, enc_mine| {
        let choice_opp = OPPONENT_CHOICE_MAP
            .get(enc_opponent)
            .expect("Exhaustive OPPONENT_CHOICE_MAP");
        let choice_mine = MY_CHOICE_MAP
            .get(enc_mine)
            .expect("Exhaustive MY_CHOICE_MAP");
        (*choice_opp, *choice_mine)
    })
}

fn process_part2(content: String) -> String {
    process_game(content, |enc_opponent, enc_mine| {
        let choice_opp = OPPONENT_CHOICE_MAP
            .get(enc_opponent)
            .expect("Exhaustive OPPONENT_CHOICE_MAP");
        let choice_mine = match enc_mine {
            "X" => {
                println!("I should lose");
                Choice::iter()
                    .find(|c| BEATS_MAP.get(choice_opp).expect("exhaustive BEATS_MAP") == c)
                    .expect("Find losing choice")
            }
            "Y" => {
                println!("I should draw");
                Choice::iter()
                    .find(|c| {
                        BEATS_MAP.get(choice_opp).expect("exhaustive BEATS_MAP") != c
                            && BEATS_MAP.get(c).expect("exhaustive BEATS_MAP") != choice_opp
                    })
                    .expect("Find drawing choice")
            }
            "Z" => {
                println!("I should win");
                Choice::iter()
                    .find(|c| BEATS_MAP.get(c).expect("exhaustive BEATS_MAP") == choice_opp)
                    .expect("Find winning choice")
            }
            &_ => panic!("Unknown enc_mine: {}", enc_mine),
        };
        (*choice_opp, choice_mine)
    })
}

pub fn main() {
    println!("Hello, AdventOfCode day 2!");
    let content = fs::read_to_string("input.txt").expect("need input");

    println!("Part 1: {}", process_part1(content.clone()));
    println!();
    println!("Part 2: {}", process_part2(content));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "A Y
B X
C Z";

    #[test]
    fn example_part1() {
        let result = process_part1(EXAMPLE.into());
        assert_eq!(result, "15");
    }

    #[test]
    fn example_part2() {
        let result = process_part2(EXAMPLE.into());
        assert_eq!(result, "12");
    }
}
