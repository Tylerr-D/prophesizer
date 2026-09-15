
// you can change this, just added this to try it :)

pub fn process_word(word: &str) -> u64 {
    word.chars().map(|c| c as u64).sum()
}
