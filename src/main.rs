pub mod output;
pub mod cli;
pub mod algs;
pub mod input;

use clap::Parser;
use cli::CliArgs;

fn main() {

    if std::env::args().nth(1).as_deref() == Some("rick")
     {
         output::sick::play_rick(); 
        return; 
    }


    let args = CliArgs::parse();

    let number = algs::process_word(&args.input);

    println!("word: {}", args.input);
    println!("number: {}", number);
    output::render(number);
}
