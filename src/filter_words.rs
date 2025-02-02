use std::str::Split;

enum FilterType {
    Equals(char),
    Excludes(char),
    Includes(char),
}

fn parse_filter_type(filter_type: char, letter: char) -> FilterType {
    match filter_type {
        '=' => FilterType::Equals(letter),
        'x' => FilterType::Excludes(letter),
        'i' => FilterType::Includes(letter),
        _ => panic!("Invalid filter type {}. Use =, x, or i.", filter_type),
    }
}

fn check_word(guess: &String, word: &String) -> bool {
    let letter_guesses: Split<&str> = guess.split(" ");
    let mut i = 0; // position
    for letter_guess in letter_guesses {
        let filter_type = parse_filter_type(
            letter_guess.chars().nth(0).unwrap(),
            letter_guess.chars().nth(1).unwrap(),
        );
        // equals
        match filter_type {
            FilterType::Equals(letter) => {
                if letter != word.chars().nth(i).unwrap() {
                    return false;
                }
            }
            FilterType::Excludes(letter) => {
                // What this actually means is that there are fewer occurances of the letter in the word than in the guess.
                // count the occurances of the letter in the guess.
                // Then count the occurances of the letter in the word.
                let guess_word: String = guess
                    .split(" ")
                    .map(|s| s.chars().nth(1).unwrap())
                    .collect();
                let letter_count_in_guess: usize =
                    guess_word.chars().filter(|c: &char| *c == letter).count();
                let letter_count_in_word: usize =
                    word.chars().filter(|c: &char| *c == letter).count();
                if letter_count_in_guess <= letter_count_in_word {
                    return false;
                }
            }
            FilterType::Includes(letter) => {
                if !word.contains(letter) {
                    return false;
                } else if word.chars().nth(i).unwrap() == letter {
                    return false;
                }
            }
        }
        // increment the counter
        i += 1;
    }
    true
}

pub fn filter_lexicon(lexicon: &Vec<String>, guesses: &Vec<String>) -> Vec<String> {
    let mut filtered_lexicon: Vec<String> = Vec::new();
    for word in lexicon.iter() {
        let mut word_is_valid = true;
        for guess in guesses.iter() {
            let is_word_valid: bool = check_word(guess, word);
            if !is_word_valid {
                word_is_valid = false;
                break;
            }
        }
        if word_is_valid {
            filtered_lexicon.push(word.clone());
        }
    }
    filtered_lexicon
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

    #[test]
    fn test_filter_lexicon() {
        let lexicon: Vec<String> = vec![
            String::from("aback"),
            String::from("raise"),
            String::from("aback"),
        ];
        let guesses: Vec<String> = vec![String::from("xr ia xi xs xe")];
        let filtered_lexicon: Vec<String> = filter_lexicon(&lexicon, &guesses);
        assert_eq!(filtered_lexicon, vec!["aback", "aback"]);
    }
}
