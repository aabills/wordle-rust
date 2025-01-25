pub fn load_lexicon() -> Vec<String> {
    let words_as_a_string =
        std::fs::read_to_string("data/all_possible_answers.txt").expect("lexicon not found...");
    let lexicon = words_as_a_string.split("\n").map(str::to_string).collect();
    lexicon
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_lexicon() {
        let lexicon = load_lexicon();
        assert_eq!(lexicon[0], "aback");
    }
}
