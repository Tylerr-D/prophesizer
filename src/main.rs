pub mod output;
pub mod cli;
pub mod algs;
pub mod input;

use clap::Parser;
use cli::CliArgs;

fn main() {
    let args = CliArgs::parse();

    if output::portraits::show_portrait(&args.input) {
        return;
    }

    if std::env::args().nth(1).as_deref() == Some("rick")
     {
         output::sick::play_rick(); 
        return; 
    }

    if args.input == "stats" {
        output::memory::stats();
        return;
    }


    let number = algs::process_word(&args.input);

    let seen = output::memory::count_word(&args.input);
    output::memory::record(&args.input);

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
