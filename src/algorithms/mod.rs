use std::process;

mod by_letter;

#[derive(Debug)]
pub enum Algorithm {
    TotalProbabilities,
    PositionalProbabilities,
    None,
}

fn parse_algorithm(algorithm: &str) -> Algorithm {
    match algorithm {
        "total-probabilities" => Algorithm::TotalProbabilities,
        "positional-probabilities" => Algorithm::PositionalProbabilities,
        "none" => Algorithm::None,
        _ => {
            println!("Invalid algorithm selected. Please select a valid algorithm.");
            process::exit(1);
        }
    }
}

pub fn get_algorithm(args: &Vec<String>) -> Algorithm {
    // Figure out the algorithm to use, default is none.
    let mut algorithm: Algorithm = Algorithm::None;
    for arg in args {
        if arg.contains("--algorithm") {
            algorithm = parse_algorithm(arg.split("=").nth(1).unwrap());
        }
    }
    println!("Algorithm: {:?}", algorithm);
    algorithm
}

pub fn run_algorithm(algorithm: &Algorithm, lexicon: &Vec<String>, _guesses: &Vec<String>) {
    match algorithm {
        Algorithm::TotalProbabilities => {
            let letter_probabilities = by_letter::total_letter_probabilities(&lexicon);
            println!("Most likely letters:");
            by_letter::print_likely_letters(&letter_probabilities);
        }
        Algorithm::PositionalProbabilities => {
            let letter_probabilities = by_letter::positional_letter_probabilities(&lexicon);
            println!("Most likely letters by position:");
            by_letter::print_likely_letters_by_position(&letter_probabilities);
            let (best_word, best_prob) =
                by_letter::positional_take_a_guess(&letter_probabilities, &lexicon);
            println!("Best word: {}", best_word);
            println!("Best probability: {}", best_prob);
        }
        Algorithm::None => {
            println!("No algorithm selected.");
        }
    }
}
