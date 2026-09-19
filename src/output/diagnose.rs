use crate::algs::process_word;
use std::time::{SystemTime, UNIX_EPOCH};

const DIAGNOSES: [&str; 4] = [
    "holy overthinking",
    "acute drama buildup",
    "suffering from too much attention",
    "verbally exhausted",
];

// i mean it doesnt obv work lol
const SYMPTOMS: [&str; 6] = [
    "too many double letters",
    "too many vowels",
    "reads the same backwards",
    "abnormally long",
    "suspiciously short",
    "all caps aggression",
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

    // lets actually make it work heheheh

    let letters: Vec<char> = word.chars().filter(|c| c.is_ascii_alphabetic()).collect();

    let s = if letters.len() == 0 {
        4
    }

    else {
        let doubles = letters.windows(2).filter(|w| w[0] == w[1]).count();
        let vowels = letters.iter().filter(|c| "aeiou" .contains(c.to_ascii_lowercase())).count();
        let lower: Vec<char> = letters.iter().map(|c| c.to_ascii_lowercase()).collect();
        let rev: Vec<char> = lower.iter().rev().cloned().collect();

        if doubles >= 2 {0}
        else if vowels >= 4 {1}
        else if lower == rev {2}
        else if word.chars().count() > 12 {3}
        else if word.chars().count() < 3 {4}
        else if word.chars().all(|c| !c.is_ascii_lowercase()) {5}
        else {(n/3) as usize % 6}
    };



    let p = (n / 5) as usize % PRESCRIPTIONS.len();

    println!("medicall reports for {}:", word);
    println!("diagonosis: {}", DIAGNOSES[d]);
    println!("symptoms: {}", SYMPTOMS[s]);
    println!("prescription: {}", PRESCRIPTIONS[p]);
}