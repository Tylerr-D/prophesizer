extern crate core;

pub mod output;
pub mod cli;
pub mod algs;
pub mod input;
pub mod other;

use clap::Parser;
use cli::CliArgs;


fn main() {
    let args = CliArgs::parse();
    
    if args.daily {
        output::daily::show_daily(args.input.as_deref())
    }


    if let Some(history) = args.history {
        output::history::show_history(&*history);
        return;
    }

    if args.input.is_none() {
        return;
    }

    let (is_secret, idx) = input::secret::check_word(args.input.clone().unwrap());

    if is_secret {
        input::secret::give_secret(idx);
        return;
    }

    if args.stats {
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
