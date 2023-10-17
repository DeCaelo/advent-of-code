#![feature(iter_array_chunks)]
use std::{collections::HashMap, fs};

// Fonction utilitaire pour créer la hashmap des scores de lettres.
fn create_letter_scores() -> HashMap<char, usize> {
    ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect()
}

pub fn process_part1(input: &str) -> String {
    let letter_scores = create_letter_scores();

    let result: usize = input
        .lines()
        .map(|line| {
            let sack_length = line.len() / 2;
            let compartment_a = &line[0..sack_length];
            let compartment_b = &line[sack_length..(sack_length * 2)];

            let common_char = compartment_a
                .chars()
                .find(|c| compartment_b.contains(*c))
                .unwrap();
            letter_scores.get(&common_char).unwrap()
        })
        .sum();

    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let letter_scores = create_letter_scores();

    let result = input
        .lines()
        .array_chunks::<3>()
        .map(|[a, b, c]| {
            let common_char = a
                .chars()
                .find(|a_char| b.contains(*a_char) && c.contains(*a_char))
                .unwrap();
            letter_scores.get(&common_char).unwrap()
        })
        .sum::<usize>();

    result.to_string()
}

pub fn main() {
    println!("Hello, AdventOfCode day 1!");
    let input = fs::read_to_string("input.txt").expect("need input");

    println!("Part 1: {}", process_part1(&input));
    println!();
    println!("Part 2: {}", process_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "157");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "70");
    }
}
