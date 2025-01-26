use std::env;
use std::fs;
use std::process;

mod filter_words;
mod guess_utilities;
mod lexicon;

//Goal 1: Print list of all compatible words
//Modes: File input, Interactive

fn main() {
    // Read input file
    let args: Vec<String> = env::args().collect();

    // Load the lexicon. Could add some customization here later
    let my_lexicon: Vec<String> = if args.contains(&String::from("--test")) {
        let my_lexicon: Vec<String> = lexicon::load_lexicon("data/test_lexicon.txt");
        my_lexicon
    } else {
        let my_lexicon: Vec<String> = lexicon::load_lexicon("data/all_possible_answers.txt");
        my_lexicon
    };

    let run_type = args[1].as_str();
    match run_type {
        r"interactive" => {
            //Just set interactive to true
            println!("Interactive mode not yet implmented. returning");
            process::exit(1);
        }
        r"file" => {
            println!("Running non-interactive mode.");

            // Go ahead and read the guess file:
            let guess_file_strings =
                fs::read_to_string(args[2].clone()).expect("Could not find input file");
            println!("Input File: \n{}", guess_file_strings);
            let guesses: Vec<String> = guess_file_strings.split('\n').map(str::to_string).collect();

            //Ensure that all guesses are valid
            for (n, guess) in guesses.iter().enumerate() {
                println!("Guess {}: {}", n, guess.clone());
                let is_guess_valid = guess_utilities::check_if_guess_is_valid(guess);
                if !is_guess_valid {
                    println!("Guess {} is invalid. Terminating program.", guess);
                    process::exit(1);
                }
            }
            println!("\n\nNew lexicon:");
            //Load the lexicon and check all words against all guesses
            for word in my_lexicon.iter() {
                let mut word_is_valid = true;
                for guess in guesses.iter() {
                    let is_word_valid = filter_words::check_word(guess, word);
                    if !is_word_valid {
                        word_is_valid = false;
                    }
                }
                if word_is_valid {
                    println!("{}", word);
                }
            }
            process::exit(0);
        }
        _ => {
            println!("Invalid run type. Please use 'interactive' or 'file'.");
            process::exit(1);
        }
    }
}
