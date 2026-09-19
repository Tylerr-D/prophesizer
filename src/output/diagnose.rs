use crate::algs::process_word;
use std::time::{SystemTime, UNIX_EPOCH};

const DIAGNOSES: [&str; 4] = [
    "holy overthinking",
    "acute drama buildup",
    "suffering from too much attention",
    "verbally exhausted",
];

// i mean it doesnt obv work lol
const SYMPTOMS: [&str; 5] = [
    "too many double letters",
    "too many vowels",
    "unexplained silence",
    "sounds worse than it looks",
    "looks worse than it sounds",
];

const PRESCRIPTIONS: [&str; 4] = [
    "one hug",
    "stop being fed to terminals",
    "rest for 42 days",
    "drink water",
];

pub fn show_diagnose(word: &str) {

        let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    let n  = process_word(word) + secs % 7;

    let d = n as usize % DIAGNOSES.len();
    let s = (n /3) as usize % SYMPTOMS.len();
    let p = (n / 5) as usize % PRESCRIPTIONS.len();

    println!("medicall reports for {}:", word);
    println!("diagonosis: {}", DIAGNOSES[d]);
    println!("symptoms: {}", SYMPTOMS[s]);
    println!("prescription: {}", PRESCRIPTIONS[p]);
}