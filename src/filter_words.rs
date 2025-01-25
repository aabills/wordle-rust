use std::fs::read_to_string;

pub fn check_word(guess: &String, word: &String) -> bool {
    let letter_guesses: Vec<String> = guess.split(" ").map(str::to_string).collect();
    let mut i = 0; // position
    for letter_guess in letter_guesses {

        // equals
        if letter_guess.chars().nth(0).unwrap() == '=' {
            if letter_guess.chars().nth(1) != word.chars().nth(i) {
                return false
            }
        }

        // excludes
        if letter_guess.chars().nth(0).unwrap() == 'x' {
            if word.contains(letter_guess.chars().nth(1).expect("test")) {
                return false
            }
        }

        // includes
        if letter_guess.chars().nth(0).unwrap() == 'i' {
            if !word.contains(letter_guess.chars().nth(1).expect("test")) {
                return false
            } else if letter_guess.chars().nth(1) == word.chars().nth(i) {
                return false
            }
        }

        // increment the counter
        i += 1;
    }
    true
}