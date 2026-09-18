use std::fs;
use std::collections::HashMap;

const LOG: &str = "prophesizer.log";

// ik its js lowk like memory.rs
// but aything to farm hours hehehehehehehe

// its cool anyways

pub(crate) fn show_history(mode: &str) {
    let data = fs::read_to_string(LOG).unwrap_or_default();

    if data.trim().is_empty() {
        println!("machine got nothing yet");
        println!("feed some words first");
        return;
    }

    let lines: Vec<&str> = data.lines().collect();
    let total = lines.len();

    match mode {
        "used" => by_used(&lines, total),
        "abc" => by_alpha(&lines, total),
        _ => by_latest(&lines, total),
    }
}

fn by_latest(lines: &[&str], total: usize) {
    println!("recent feedings:");
    println!();

    let start = total.saturating_sub(10);

    for line in &lines[start..] {
        println!("  {}", line);
    }

    footer(total);
}

fn by_used(lines: &[&str], total: usize) {
    let mut counts: HashMap<&str, u32> = HashMap::new();

    for lines in lines {
        *counts.entry(lines).or_insert(0) += 1;
    }

    let mut pairs: Vec<(&str, u32)> = counts.into_iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1));

    println!("most consulted:");println!();

    for (word, count) in pairs.iter().take(10) {
        println!("{:>3} x  {}", count, word);
    }

    footer(total);
}

fn by_alpha(lines: &[&str], total: usize) {
    let mut counts: HashMap<&str, u32> = HashMap::new();

    for lines in lines {
        *counts.entry(lines).or_insert(0) += 1;
    }

    let mut pairs: Vec<(&str, u32)> = counts.into_iter().collect();
    pairs.sort_by(|a, b| a.0.cmp(b.0));

    println!("the word list, alphabetized:");println!();

    for(word, count) in &pairs {
        println!("  {:>3} x  {}", count, word);
    }

    footer(total);
}

fn footer(total: usize) {
    println!();
    println!("total words fed: {}", total);

    if total > 20 {
        println!("that is a lot of words, do you use this machine or worship it");
    }
}
