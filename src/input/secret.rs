use std::collections::HashMap;
use std::sync::LazyLock;
use crate::input::Secrets;

const SECRET_FILE: &'static str = include_str!("secret.json");

pub(crate) static SECRET_WORDS: LazyLock<HashMap<usize, Secrets>> = LazyLock::new(|| {
    serde_json::from_str(SECRET_FILE).unwrap()
});

pub(crate) fn check_word(input: String) -> (bool, usize) {
    let mut index: usize = -1;
    let mut matched: bool = false;

    for (i, secret) in SECRET_WORDS.iter() {
        if input == secret.input.as_deref().unwrap_or("") {
            index = *i;
            matched = true;
            break;
        }
    }

    (matched, index)
}

fn give_secret(index: usize) {
    let secret = SECRET_WORDS.get(&index).unwrap();

    if secret.category.is_some() {
        let category = secret.category.as_ref().unwrap_or(&"".to_string());

        if category == "text" {
            if secret.outputs.is_some() {
                let outputs = secret.outputs.as_ref().unwrap();
                todo!(
                    "call output::secret::output_variable_secret"
                )
            } else if secret.output.is_some() {
                let output = secret.output.as_ref().unwrap();
                todo!(
                    "call output::secret::output_secret"
                )
            } else {
                todo!(
                    "call generic output"
                )
            }
        }

        if category == "ascii-animation" {
            todo!(
                "call output::secret::ascii-animation"
            )
        }

        if category == "ascii-art" {
            todo!(
                "call output::ascii-art"
            )
        }

        // add more categories here
    }
}
