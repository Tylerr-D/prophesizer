use std::sync::LazyLock;
use csv::ReaderBuilder;

const SECRET_CSV: &str = include_str!("secret.json");

pub(crate) struct Secrets {
    id: usize,
    word: String,
    description: String,
}

// old csv code, ignore

pub(crate) static SECRET_WORDS: LazyLock<Vec<String>> = LazyLock::new(|| {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(SECRET_CSV.as_bytes());

    reader
        .records()
        .filter_map(|result| result.ok())
        .map(|record| record.get(0).unwrap().to_string())
        .collect()
});

pub(crate) fn find(input: String) {
    // more csv, need to switch to json

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(SECRET_CSV.as_bytes());

    let record_count = reader.records().count();
    todo!(
        "call check_words, then after that if not secret, then dont do anything, if it is secret, then call the secret outputs"
    )
}

fn check_words(word: String) -> bool {
    todo!(
        "loop csv to check if word exists"
    )
}
