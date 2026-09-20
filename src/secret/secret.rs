use std::collections::HashMap;
use std::sync::LazyLock;

use crate::secret::Secrets;
use crate::output;
use crate::output::type_out;

const _SECRET_FILE: &'static str = include_str!("secret.json");

pub(crate) static _SECRET_WORDS: LazyLock<HashMap<usize, Secrets>> = LazyLock::new(|| {
    serde_json::from_str(_SECRET_FILE).unwrap()
});

pub(crate) fn check_word(input: String) -> (bool, usize) {
    let mut index: usize = 0;
    let mut matched: bool = false;

    for (i, secret) in _SECRET_WORDS.iter() {
        if secret.inputs.is_some() {
            let inputs: Vec<String> = secret.inputs.as_deref().unwrap().to_vec();

            for sub_input in inputs.iter() {
                if sub_input == input.as_str() {
                    index = *i;
                    matched = true;
                    break;
                }
            }
        }

        else if input == secret.input.as_deref().unwrap_or("") {
            index = *i;
            matched = true;
            break;
        }
    }

    (matched, index)
}

pub(crate) fn give_secret(index: usize) {

    let secret = _SECRET_WORDS.get(&index).unwrap();

    if secret.category.is_some() {
        let empty: &String = &String::from("");
        let category: &str = secret.category.as_ref().unwrap_or(empty);

        if category == "text" {
            if secret.outputs.is_some() {
                let outputs = secret.outputs.as_ref().unwrap();
                output::secret::output_variable_secret(outputs)
            }

            else if secret.output.is_some() {
                let output = secret.output.as_ref().unwrap();
                output::secret::output_secret(output)
            }

            else {
                todo!(
                    "call generic output"
                )
            }
        }

        else if category == "ascii-animation" {
            output::secret::ascii_animation(secret.input.clone().unwrap_or_default());
        }

        else if category == "ascii-art" {
            output::secret::ascii_art(secret.input.clone().unwrap_or_default());
        }

        else if category == "portrait" {
            output::secret::show_portrait(secret.input.clone().unwrap_or_default());
        }

        else {
            type_out("", true);
            todo!(
                "call generic output"
            )
        }
    }
}
