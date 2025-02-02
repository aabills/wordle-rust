pub fn total_letter_probabilities(lexicon: &Vec<String>) -> Vec<f64> {
    // Count the letters
    let mut letter_counts: Vec<i32> = vec![0; 26];
    for word in lexicon {
        let one_hot_guesses = convert_word_to_one_hot(word);
        let total_sum = total_word_one_hot_sum(&one_hot_guesses);
        for (i, &count) in total_sum.iter().enumerate() {
            letter_counts[i] += count;
        }
    }

    // Calculate the probabilities
    let total_letters = 5.0 * lexicon.len() as f64;
    let probabilities = letter_counts
        .iter()
        .map(|&count| count as f64 / total_letters)
        .collect();
    probabilities
}

pub fn positional_letter_probabilities(lexicon: &Vec<String>) -> Vec<Vec<f64>> {
    let mut positional_letter_counts: Vec<Vec<i32>> = vec![vec![0; 26]; 5];
    for word in lexicon {
        let one_hot_guesses = convert_word_to_one_hot(word);
        for (i, guess) in one_hot_guesses.iter().enumerate() {
            for (j, &letter_one_hot) in guess.iter().enumerate() {
                positional_letter_counts[i][j] += letter_one_hot;
            }
        }
    }

    // Calculate the probabilities
    let total_letters = lexicon.len() as f64;
    let probabilities = positional_letter_counts
        .iter()
        .map(|count| count.iter().map(|&c| c as f64 / total_letters).collect())
        .collect();
    probabilities
}

fn convert_letter_guess_to_one_hot(letter_guess: char) -> Vec<i32> {
    let mut one_hot: Vec<i32> = vec![0; 26];
    let letter_guess_index: i32 = letter_guess as i32 - 97;
    one_hot[letter_guess_index as usize] = 1;
    one_hot
}

fn total_word_one_hot_sum(one_hot_guesses: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut total_sum: Vec<i32> = vec![0; 26];
    for guess in one_hot_guesses {
        for (i, &letter_one_hot) in guess.iter().enumerate() {
            total_sum[i] += letter_one_hot;
        }
    }
    total_sum
}

fn convert_word_to_one_hot(word: &str) -> Vec<Vec<i32>> {
    let mut one_hot_guesses: Vec<Vec<i32>> = Vec::new();
    for letter_guess in word.chars() {
        one_hot_guesses.push(convert_letter_guess_to_one_hot(letter_guess));
    }
    one_hot_guesses
}

pub fn print_likely_letters(probabilities: &Vec<f64>) {
    let mut sorted_probabilities = probabilities
        .iter()
        .enumerate()
        .collect::<Vec<(usize, &f64)>>();
    sorted_probabilities.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    for (i, &prob) in sorted_probabilities {
        println!("{}: {:.3}", (i as u8 + 97) as char, prob);
    }
}

pub fn print_likely_letters_by_position(probabilities: &Vec<Vec<f64>>) {
    let num_letters = 26;
    for ranking in 0..num_letters {
        for position in 0..5 {
            let mut sorted_probabilities = probabilities[position]
                .iter()
                .enumerate()
                .collect::<Vec<(usize, &f64)>>();
            sorted_probabilities.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
            let (i, &prob) = sorted_probabilities[ranking];
            print!("| {}: {:.3} |", (i as u8 + 97) as char, prob);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_letter_guess_to_one_hot() {
        let letter_guess: &str = "ia";
        let one_hot: Vec<i32> =
            convert_letter_guess_to_one_hot(letter_guess.chars().nth(1).unwrap());
        assert_eq!(
            one_hot,
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }
}
