use serde::Deserialize;

pub mod input;
pub mod secret;

#[derive(Deserialize)]
pub(crate) struct Secrets {
    id: Option<usize>,

    #[serde(rename="word")]
    input: Option<String>,
    description: Option<String>,
    output: Option<String>,
    outputs: Option<Vec<String>>,
    category: Option<String>,
}
