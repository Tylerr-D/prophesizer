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

    if args.karma {
        other::karma::show();
        return;
    }
    
    if args.daily {
        output::daily::show_daily(args.input.as_deref())
    }


    if let Some(history) = args.history {
        output::history::show_history(&*history);
        return;
    }

    if args.stats {
        other::memory::stats();
        return;
    }

    if let Some(word) = args.diagnose {
        output::diagnose::show_diagnose(word.as_str());
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

    other::memory::stat_generic(args);
}
