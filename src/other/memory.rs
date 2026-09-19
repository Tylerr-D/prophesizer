use std::collections::HashMap;
use std::fs;

use crate::{algs, output};
use crate::cli::CliArgs;

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
    crate::other::karma::feed(word);
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

    crate::other::karma::show();
}

pub(crate) fn stat_generic(args: CliArgs) {
    if args.input.is_none() {
        return;
    }

    let input = args.input.as_deref().unwrap().trim();

    let number = algs::process_word(input);

    let seen = count_word(input);
    record(input);

    println!();

    println!("word: {}", input);
    println!("number: {}", number);
    output::render(number);

    println!();

    if seen > 0 {
        println!("hmm, i heard this before...");
        println!("the machine remembers \"{}\" has been fed {} time{} before",
                 input, seen, if seen>1 {"s"} else {""});

        println!();

        if seen == 2 {
            println!("gng obsession is a sin");
        }

        if seen >= 5 {
            println!("this word lives here rent free");
        }

        if seen == 67 {
            println!("SIX SEVENNN");
            println!("but srsly, do you think something different will happen?")
        }
    }

    let (kscore, ktier) = crate::other::karma::state(input);
        println!("karma: {} ({})", ktier, kscore);
}