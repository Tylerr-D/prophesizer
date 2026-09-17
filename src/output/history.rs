use std::fs;
use std::collections::HashMap;

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

    match mode {

        "used" => by_used(&lines, total),
        "abc" => by_alpha(&lines, total),
        _ => by_latest(&lines, total),
    }

}

    // println!();
    // println!("total words fed: {}", total);

    // if total > 20 {
    //     println!("that is a lot of words, do you use this machine or worship it");
    // }

fn by_latest(lines: &|&str|, total: usize){

    println!("recent feedings:");
    println!();

    let start = total.saturating_sub(10);

    or line in &lines[start..] {
         println!("  {}", line);
     }

     footer(total);
}

fn by_used(lines: &|&str|, total: usize){

    let mut counts: HashMap<&str, u32> = HashMap::new();

    let lines in lines {
        *counts.entry(line).or_insert(0) += 1;
    }
}