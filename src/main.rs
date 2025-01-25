use std::env;
use std::fs;

mod filter_words;
mod guess_utilities;
mod lexicon;
mod utilities;

//Goal 1: Print list of all compatible words
//Modes: File input, Interactive

fn main() {
    // Read input file
    let args: Vec<String> = env::args().collect();

    // Load the lexicon. Could add some customization here later
    let my_lexicon = lexicon::load_lexicon();

    if args[1] == "interactive" {
        //Just set interactive to true
        println!("Running interactive mode");
    } else if args[1] == "file" {
        println!("Running non-interactive mode.");

        // Go ahead and read the guess file:
        let guess_file_strings =
            fs::read_to_string(args[2].clone()).expect("Could not find input file");
        println!("Input File: \n{}", guess_file_strings);
        let guesses: Vec<String> = guess_file_strings.split('\n').map(str::to_string).collect();

        //Ensure that all guesses are valid
        for (n, guess) in guesses.iter().enumerate() {
            println!("Guess {}: {}", n, guess.clone());
            guess_utilities::check_if_guess_is_valid(guess);
        }
        //Spacing for aesthetics
        for _n in 0..2 {
            println!("");
        }
        println!("New lexicon:");
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
    }
}
