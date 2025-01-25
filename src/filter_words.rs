pub fn check_word(guess: &String, word: &String) -> bool {
    let letter_guesses: Vec<String> = guess.split(" ").map(str::to_string).collect();
    let mut i = 0; // position
    for letter_guess in letter_guesses {
        // equals
        if letter_guess.chars().nth(0).unwrap() == '=' {
            if letter_guess.chars().nth(1) != word.chars().nth(i) {
                return false;
            }
        }

        // excludes
        if letter_guess.chars().nth(0).unwrap() == 'x' {
            // What this actually means is that there are fewer occurances of the letter in the word than in the guess.
            // count the occurances of the letter in the guess.
            // Then count the occurances of the letter in the word.
            let letter_count_in_guess: usize = guess
                .chars()
                .filter(|c: &char| *c == letter_guess.chars().nth(1).unwrap())
                .count();
            let letter_count_in_word: usize = word
                .chars()
                .filter(|c: &char| *c == letter_guess.chars().nth(1).unwrap())
                .count();
            if letter_count_in_guess <= letter_count_in_word {
                return false;
            }
        }

        // includes
        if letter_guess.chars().nth(0).unwrap() == 'i' {
            if !word.contains(letter_guess.chars().nth(1).expect("test")) {
                return false;
            } else if letter_guess.chars().nth(1) == word.chars().nth(i) {
                return false;
            }
        }

        // increment the counter
        i += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_word() {
        // Assume that the real word is "aback", and our first guess is "xr ia xi xs xe"
        // Then the lexicon should still contain aback!
        let word: String = String::from("aback");
        let guess: String = String::from("xr ia xi xs xe");
        assert!(check_word(&guess, &word));

        // Here's a trickier one:
        let word: String = String::from("raise");
        let guess: String = String::from("ia xb xa xc xk");
        assert!(check_word(&guess, &word));
    }
}
