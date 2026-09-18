use std::collections::HashMap;
use std::fs;

const LOG: &str = "prophesizer.log";

pub fn count_word(word: &str) -> u32 {

    let data = fs::read_to_string(LOG).unwrap_or_default();
    data.lines().filter(|line| *line == word).count() as u32 

}

pub fn record(word:&str) {
    let mut data = fs::read_to_string(LOG).unwrap_or_default();
    data.push_str(word);
    data.push('\n');
    let _ = fs::write(LOG, data);
    crate::other::days::feed();
}

pub fn stats() {
    let data = fs::read_to_string(LOG).unwrap_or_default();

    if data.trim().is_empty() {
        println!("machine remembered nothing");
        println!("feed it words first");
        return;
    }

    let mut counts: HashMap<&str, u32> = HashMap::new();

    for line in data.lines() {
        *counts.entry(line).or_insert(0) += 1;
    }

    let mut top_word = "";

    let mut top_count = 0;

    for (word, count) in &counts {
        if *count > top_count {
            top_count = *count;

            top_word = word;
        }
    }


    println!("total readings: {}", data.lines().count());
    println!("different words: {}", counts.len());
    println!("most-consulted: \"{}\" ({} times)", top_word, top_count);

    let days = crate::other::days::streak();
    println!("fed {} day{} in a row", days, if days == 1 { "" } else { "s" });

    if top_count >= 5 {
        println!("yea gng get over this word already. the obsession is crazy");

    }
}
