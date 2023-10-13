use std::fs;
use std::io;

fn calculate_score(line: &str) -> u32 {
    // Transformer A, B, C et X, Y, Z en 0, 1, 2 respectivement en utilisant leur ordre ASCII
    let left = match line.as_bytes()[0] {
        b'A' => 0,
        b'B' => 1,
        b'C' => 2,
        _ => unreachable!(),
    };

    let right = match line.as_bytes()[2] {
        b'X' => 0,
        b'Y' => 1,
        b'Z' => 2,
        _ => unreachable!(),
    };

    let my_shape = right as i8;
    let opponent_shape = left as i8;
    let outcome = (my_shape - opponent_shape + 1).rem_euclid(3);

    let shape_score = my_shape + 1;
    let outcome_score = 3 * outcome;

    (shape_score + outcome_score) as u32
}

fn main() -> io::Result<()> {
    // Lire le contenu du fichier "input" dans une chaîne de caractères
    let input = fs::read_to_string("input")?;
    // Gérer les erreurs liées à la lecture du fichier avec ?
    // Parcourir chaque ligne du fichier, calculer le score pour chaque tour et sommer les scores
    let total_score: u32 = input.lines().map(calculate_score).sum();

    println!("{}", total_score);
    Ok(())
}

// La méthode rem_euclid est une fonction dans Rust qui permet de calculer
// le reste de la division euclidienne d'un nombre par un autre.
// En mathématiques, la division euclidienne est une opération qui divise un nombre en deux parties :
// le quotient et le reste. Le reste de la division euclidienne est toujours un nombre positif ou nul.

// Dans le code que vous avez fourni, rem_euclid(3) est utilisé pour calculer
// le reste de la division euclidienne de (my_shape - opponent_shape + 1) par 3.
// Cela permet de normaliser le résultat de manière à ce qu'il soit toujours compris entre 0, 1 et 2.

// Voici comment cela fonctionne :

// Si my_shape - opponent_shape + 1 est égal à 0, le reste de la division euclidienne par 3 sera 0.
// Si my_shape - opponent_shape + 1 est égal à 1, le reste de la division euclidienne par 3 sera 1.
// Si my_shape - opponent_shape + 1 est égal à 2, le reste de la division euclidienne par 3 sera 2.
// Cela permet de représenter les trois résultats possibles du jeu (défaite, égalité, victoire)
// avec des valeurs de 0 à 2, ce qui peut être plus facile à manipuler dans le code.
