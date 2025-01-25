pub fn check_if_guess_is_valid(guess: &String) -> bool {
    // split the guess into
    let split_guess: Vec<String> = guess.split(' ').map(str::to_string).collect();
    let guess_len: usize = split_guess.len();
    if guess_len != 5 {
        println!("Guess {} is invalid. Guesses must be of length 5.", guess);
        return false;
    }
    let possible_prefixes: [char; 3] = ['i', 'x', '='];
    for letter_guess in split_guess {
        let first_letter_of_guess: char = letter_guess.chars().nth(0).expect("Issue reading guess");
        if possible_prefixes.contains(&first_letter_of_guess) {
            // pass
        } else {
            println!("Guess {} letter {} cannot be the first letter of a guess, the first letter must be i, x, or =", guess, first_letter_of_guess);
            return false;
        }
        let second_letter_of_guess: char =
            letter_guess.chars().nth(1).expect("Issue reading guess");

        //Assert that the second letter is A-Z
        let second_letter_of_guess_int: i8 = second_letter_of_guess as i8;
        if (second_letter_of_guess_int < 97) | (second_letter_of_guess_int > 122) {
            println!("Guess {} letter {} cannot be the second letter of a guess, the second letter must be a lowercase a-z", guess, second_letter_of_guess);
            return false;
        }

        let letter_guess_length = letter_guess.len();
        //Assert that the guess is length 2
        if letter_guess_length != 2 {
            println!(
                "Part {} of guess {} is invalid because it is not the correct length.",
                letter_guess, guess
            );
            return false;
        }
    }

    println!("Guess {} is valid!", guess);
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_if_guess_is_valid() {
        assert!(check_if_guess_is_valid(&String::from("xr ia xi xs xe")));
    }
    #[test]
    fn test_check_if_guess_is_invalid_incorrect_indicator() {
        assert!(!check_if_guess_is_valid(&String::from("qr ia xi xs xe")));
    }

    #[test]
    fn test_check_if_guess_is_invalid_incorrect_length() {
        assert!(!check_if_guess_is_valid(&String::from("xr ia xi xs")));
    }

    #[test]
    fn test_check_if_guess_is_invalid_incorrect_second_letter() {
        assert!(!check_if_guess_is_valid(&String::from("xr ia xi xs x?")));
    }

    #[test]
    fn test_check_if_guess_is_invalid_incorrect_length_of_letter() {
        assert!(!check_if_guess_is_valid(&String::from("xrx ia xi xs x")));
    }
}
