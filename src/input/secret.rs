use std::collections::HashMap;
use std::sync::LazyLock;
use crate::input::Secrets;

const SECRET_FILE: &'static str = include_str!("secret.json");

pub(crate) static SECRET_WORDS: LazyLock<Vec<Secrets>> = LazyLock::new(|| {
    serde_json::from_str(SECRET_FILE).unwrap()
});

pub(crate) fn find(input: String) {
    let mut special_words: HashMap<usize, &str> = HashMap::new();
    let mut index = 0;
    for secret in SECRET_WORDS.iter() {
        special_words.insert(index, secret.input.as_deref().unwrap_or(""));
        index += 1;
    }
}

fn check_words(word: String) -> bool {
    todo!(
        "loop csv to check if word exists"
    )
}
