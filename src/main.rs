pub mod output;
pub mod cli;
pub mod algs;
pub mod input;

use clap::Parser;
use cli::CliArgs;

fn main() {
    let args = CliArgs::parse();

    let number = algs::process_word(&args.input);

    println!("word: {}", args.input);
    println!("number: {}", number);
    output::render(number);
}
