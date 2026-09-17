extern crate core;

pub mod output;
pub mod cli;
pub mod algs;
pub mod input;
pub mod other;

use clap::Parser;
use cli::CliArgs;

fn main() {

    let mut argv = std::env::args();
    let cmd = argv.nth(1);

    if cmd.as_deref() == Some("daily") {
        let word = argv.next().unwrap_or_else(|| String::from("the day"));
    output::daily::show_daily(&word);
        return;

    }

    if cmd.as_deref() == Some("history") {
        let mode = argv.next().unwrap_or_default();
        output::history::show_history(&mode);
        return;
    }


    let args = CliArgs::parse();

    if output::portraits::show_portrait(&args.input) {
        return;
    }

    let first = std::env::args().nth(1);
    if let Some(word) = &first {
        if matches!(word.as_str(), "rick" | "lou" | "sick" | "prophesizer") {
            output::sick::play_rick();
            return;
        }

        if args.input == "stats" {
            other::memory::stats();
            return;
        }

        other::memory::stats();

        let number = algs::process_word(&args.input);

        let seen = other::memory::count_word(&args.input);
        other::memory::record(&args.input);

        println!();

    println!("word: {}", args.input);
    println!("number: {}", number);
    output::render(number);

    println!();

        if seen > 0 {
            println!("hmm, i heard this before...");
            println!("the machine remembers \"{}\" has been fed {} time{} before",
                     args.input, seen, if seen>1 {"s"} else {""});
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
    }
}
