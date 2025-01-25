use crate::utilities;

pub fn check_if_guess_is_valid(guess: &String){
    // split the guess into 
    let split_guess: Vec<_> = guess.split(' ').collect();
    let guess_len = split_guess.len();
    if guess_len != 5 {
        println!("Guess {} is invalid. Guesses must be of length 5.", guess);
    }
    let possible_prefixes = ['i', 'x', '='];
    for letter_guess in &split_guess {
        let first_letter_of_guess = letter_guess.chars().nth(0).expect("Issue reading guess");
        if possible_prefixes.contains(&first_letter_of_guess) {
            // pass
        } else {
            println!("Guess {} letter {} cannot be the first letter of a guess, the first letter must be i, x, or =", guess, first_letter_of_guess);
        }
        let second_letter_of_guess = letter_guess.chars().nth(1).expect("Issue reading guess");
        
        //Assert that the second letter is A-Z
        let second_letter_of_guess_int = second_letter_of_guess as i8;
        if (second_letter_of_guess_int < 97) | (second_letter_of_guess_int > 122) {
            println!("Guess {} letter {} cannot be the second letter of a guess, the second letter must be a lowercase a-z", guess, second_letter_of_guess);
        } 

        let letter_guess_length = letter_guess.len();
        //Assert that the guess is length 2
        if letter_guess_length != 2 {
            println!("Part {} of guess {} is invalid because it is not the correct length.", letter_guess, guess)
        }

    }

    println!("Guess {} is valid!", guess);
}