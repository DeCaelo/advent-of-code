use std::fs;

pub fn process_part1(input: &str) -> String {
    let result = input
        .split_whitespace()
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut result = input
        .split_whitespace()
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    result.sort_by(|a, b| b.cmp(a));
    let sum: u32 = result.iter().take(3).sum();
    sum.to_string()
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

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "24000");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "45000");
    }
}

// Utilisation de split_whitespace au lieu de split("\n\n") et split("\n") :
// Cela simplifie la séparation des nombres, car split_whitespace divise automatiquement
// le texte en utilisant n'importe quel espace (y compris les sauts de ligne).

// Utilisation de iter().max() :
// Cela permet de trouver facilement la valeur maximale dans le vecteur counts sans trier le vecteur.

// fn main() {
//     match fs::read_to_string("input") {
//         Ok(content) => {
//             let counts: Vec<usize> = content
//                 .split_whitespace()
//                 .map(|s| s.parse().unwrap_or(0))
//                 .collect();

//             match counts.iter().max() {
//                 Some(max_count) => println!("The highest count is {}", max_count),
//                 None => println!("No valid counts found in the input file."),
//             }
//         }
//         Err(err) => eprintln!("Error reading the input file: {}", err),
//     }
// }
