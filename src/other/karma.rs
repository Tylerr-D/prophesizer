use std::collections::HashMap;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

const FILE: &str = "prophesizer.karma";

fn today() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() / 86_400
}

fn read() -> HashMap<String, (u32, u64)> {
    let mut map = HashMap::new();
    for line in fs::read_to_string(FILE).unwrap_or_default().lines() {
        let mut it = line.splitn(3, ' ');
        let w = it.next();
        let c = it.next().and_then(|s| s.parse().ok());
        let d = it.next().and_then(|s| s.parse().ok());
        if let (Some(w), Some(c), Some(d)) = (w, c, d) {
            map.entry(w.to_string()).or_insert((c, d));
        }
    }
    map
}

fn write(map: &HashMap<String, (u32, u64)>) {
    let mut data = String::new();
    for (w, (c, d)) in map {
        data.push_str(&format!("{} {} {}\n", w, c, d));
    }
    let _ = fs::write(FILE, data);
}

pub fn feed(word: &str) {
    let mut map = read();
    let (c, d) = map.entry(word.to_string()).or_insert((0, today()));
    *c += 1;
    *d = today();
}

pub fn tier(score: i64) -> &'static str {
    if score >= 3 { "blessed" }
    else if score >= 1 { "warm" }
    else if score >= 0 { "cold" }
    else { "forgotten" }
}

pub fn state(word: &str) -> (i64, &'static str) {
    let map = read();
    let (c, d) = map.get(word).copied().unwrap_or((0, today()));
    let score = c as i64 - (today() as i64 - d as i64);
    (score, tier(score))
}

pub fn show() {
    let map = read();
    let mut rows: Vec<(String, i64)> = map
        .iter()
        .map(|(w, (c, d))| (w.clone(), *c as i64 - (today() as i64 - *d as i64)))
        .collect();
    rows.sort_by(|a, b| b.1.cmp(&a.1));

    if rows.is_empty() {
        println!("the machine holds no grudges yet");
        return;
    }

    println!();
    println!("karma:");
    for (w, s) in rows {
        println!("  {:>5}  {}  {}", s, tier(s), w);
    }
}
