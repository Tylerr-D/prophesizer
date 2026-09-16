use std::collections::HashMap;
use std::sync::LazyLock;
use crate::input::Secrets;

const SECRET_FILE: &'static str = include_str!("secret.json");

pub(crate) static SECRET_WORDS: LazyLock<Vec<Secrets>> = LazyLock::new(|| {
    serde_json::from_str(SECRET_FILE).unwrap()
});

pub(crate) fn find(input: String) -> bool, usize {
    let mut index: usize = -1;
    let mut matched: bool = false;

    for (i, secret) in SECRET_WORDS.iter() {
        if input == secret.input.as_deref().unwrap_or("") {
            index = *i;
            matched = true;
            break;
        }
    }

    matched, index
}

fn check_words(word: String) -> bool {
    todo!(
        "loop csv to check if word exists"
    )
}
