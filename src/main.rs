use std::env;
use std::fs;
use std::io;
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
            println!("Now running in interactive mode.\n\n");
            let mut guesses: Vec<String> = Vec::new();
            // Loop until the user wants to quit
            loop {
                println!("Enter a guess: ");

                //Read the guess
                let mut guess: String = String::new();
                io::stdin()
                    .read_line(&mut guess)
                    .expect("Failed to read line");
                //Trim the newline character
                guess = guess.trim().to_string();

                //Add the guess to the list of guesses
                guesses.push(guess);

                //Print the new lexicon
                print_new_lexicon(&my_lexicon, &guesses);

                //Ask the user if they want to continue
                println!("Enter 'q' to quit, or 'c' to continue");
                let mut continue_input: String = String::new();
                io::stdin()
                    .read_line(&mut continue_input)
                    .expect("Failed to read line");
                if continue_input.contains("q") {
                    process::exit(0);
                }
            }
        }
        r"file" => {
            println!("Running non-interactive mode.");

            // Read the guess file
            let filepath: String = args[2].clone();
            let guess_file_strings: String =
                fs::read_to_string(filepath).expect("Could not find input file");
            println!("Input File: \n{}", guess_file_strings);
            let guesses: Vec<String> = guess_file_strings.split('\n').map(str::to_string).collect();

            print_new_lexicon(&my_lexicon, &guesses);
            process::exit(0);
        }
        _ => {
            println!("Invalid run type. Please use 'interactive' or 'file'.");
            process::exit(1);
        }
    }
}

fn print_new_lexicon(lexicon: &Vec<String>, guesses: &Vec<String>) {
    //Ensure that all guesses are valid
    guess_utilities::check_guesses(&guesses);

    //Load the lexicon and check all words against all guesses
    let filtered_lexicon: Vec<String> = filter_words::filter_lexicon(&lexicon, &guesses);

    //Print the new lexicon
    println!("\n\nNew lexicon:");
    println!("{}", filtered_lexicon.join("\n"));
}
