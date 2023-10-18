#![feature(iter_array_chunks)]
use std::fs;

pub fn main() {
    println!("Hello, AdventOfCode day 3!");
    let content = fs::read_to_string("./input").expect("Read ./input");

    println!("PART 1 - RESULT: {}", process_part1(content.clone()));
    println!();
    println!("PART 2 - RESULT: {}", process_part2(content));
}

fn process_part1(content: String) -> String {
    let mut duplicates_prio_sum = 0;
    for line in content.lines() {
        let (first, second) = line.split_at(line.len() / 2);
        let duplicate = find_duplicates(vec![first, second]);
        let duplicate_prio = get_priority(duplicate);
        println!(
            "{} | {} - dup={}, prio={}",
            first, second, duplicate, duplicate_prio
        );
        duplicates_prio_sum += duplicate_prio;
    }
    duplicates_prio_sum.to_string()
}

fn process_part2(content: String) -> String {
    let mut badge_prio_sum = 0;
    for team in content.lines().array_chunks::<3>() {
        let badge = find_duplicates(team.into());
        let duplicate_prio = get_priority(badge);
        println!("{:?} - dup={}, prio={}", team, badge, duplicate_prio);
        badge_prio_sum += duplicate_prio;
    }
    badge_prio_sum.to_string()
}

fn find_duplicates(sets: Vec<&str>) -> char {
    let chars_of_other_sets: Vec<Vec<char>> = sets
        .iter()
        .skip(1)
        .map(|set| set.chars().collect::<Vec<_>>())
        .collect();
    // we iterate through the chars of the first set and check if all other sets contain it
    sets[0]
        .chars()
        .find(|c| {
            chars_of_other_sets
                .iter()
                .all(|other_set| other_set.contains(c))
        })
        .expect(format!("No duplicates in: {:?}", sets).as_str())
}

fn get_priority(c: char) -> u16 {
    match c.is_uppercase() {
        false => 1 + (c as u16) - ('a' as u16),
        true => 27 + (c as u16) - ('A' as u16),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn example_priorities() {
        assert_eq!(get_priority('p'), 16);
        assert_eq!(get_priority('L'), 38);
        assert_eq!(get_priority('P'), 42);
    }

    #[test]
    fn example_part1() {
        let result = process_part1(EXAMPLE.into());
        assert_eq!(result, "157");
    }

    #[test]
    fn real_part1() {
        let result = process_part1(fs::read_to_string("./input").expect("Read ./input"));
        assert_eq!(result, "7917");
    }

    #[test]
    fn example_part2() {
        let result = process_part2(EXAMPLE.into());
        assert_eq!(result, "70");
    }

    #[test]
    fn real_part2() {
        let result = process_part2(fs::read_to_string("./input").expect("Read ./input"));
        assert_eq!(result, "2585");
    }
}
