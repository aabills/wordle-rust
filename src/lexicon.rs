pub fn load_lexicon(path: &str) -> Vec<String> {
    let words_as_a_string = std::fs::read_to_string(path).expect("lexicon not found...");
    let lexicon = words_as_a_string.split("\n").map(str::to_string).collect();
    lexicon
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_lexicon() {
        let lexicon: Vec<String> = load_lexicon("data/test_lexicon.txt");
        assert_eq!(lexicon[0], "aback");
    }
}
