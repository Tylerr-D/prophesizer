use std::collections::BTreeSet;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

const DAYS_FILE: &str = "prophesizer.days";

fn today() -> u64 {

    SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs()
    / 86_400
}

pub fn feed(){
    let mut data = fs::read_to_string(DAYS_FILE).unwrap_or_default();
    data.push_str(&today().to_string());
    data.push('\n');
    let _ = fs::write(DAYS_FILE, data);
}

pub fn streak() -> u32 {

    let days: BTreeSet<u64> = fs::read_to_string(DAYS_FILE)
        .unwrap_or_default()
        .lines()
        .filter_map(|l| l.trim().parse().ok())
        .collect();

        let t = today();
        let start = if days.contains(&t) { t }
         else {
             t.saturating_sub(1) 
            };

let mut n = 0;
let mut day = start;
while days.contains(&day) {
    n +=1;
    day = day.saturating_sub(1);
}
n
}