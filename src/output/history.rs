use std::fs;

const LOG: &str = "prophesizer.log";

// ik its js lowk like memory.rs
// but aything to farm hours hehehehehehehe

pub fn show_history(){

    let data = fs::read_to_string(LOG).unwrap_or_default();

    if data.trim().is_empty(){

        println!("machine got nothing yet");
        println!("feed some words first");
        return;
    }

    let lines: Vec<&str> = data.lines().collect();
    let total = lines.len();

    println!("recent feedings:");
    println!();

    let start = total.saturating_sub(10);

    for line in &lines[start..] {
        println!("  {}", line);
    }

    println!();
    println!("total words fed: {}", total);

    if total > 20 {
        println!("that is a lot of words, do you use this machine or worship it");
    }

}