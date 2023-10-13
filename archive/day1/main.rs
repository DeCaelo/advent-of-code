use std::fs;

fn main() {
    match fs::read_to_string("input") {
        Ok(content) => {
            let counts: Vec<usize> = content
                .split_whitespace()
                .map(|s| s.parse().unwrap_or(0))
                .collect();

            match counts.iter().max() {
                Some(max_count) => println!("The highest count is {}", max_count),
                None => println!("No valid counts found in the input file."),
            }
        }
        Err(err) => eprintln!("Error reading the input file: {}", err),
    }
}

// Utilisation de split_whitespace au lieu de split("\n\n") et split("\n") :
// Cela simplifie la séparation des nombres, car split_whitespace divise automatiquement
// le texte en utilisant n'importe quel espace (y compris les sauts de ligne).

// Utilisation de iter().max() :
// Cela permet de trouver facilement la valeur maximale dans le vecteur counts sans trier le vecteur.
