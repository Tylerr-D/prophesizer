

use std::collections::HashMap;
use std::fs;

const LOG &str = "prophesizer.log";

pub fn count_word(word: &str) -> u32 {

    let data = fs::read_to_string(LOG).unwrap_or_default();
    data.lines().filter(|line| *line == word).count() as u32 

}

pub fn record(word:&str) {
    let mut data = fs::read_to_string(LOG).unwrap_or_default();
    data.push_str(word);
    data.push('\n');
        let _ = fs::write(LOG, data);
}