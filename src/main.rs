extern crate core;

pub mod output;
pub mod cli;
pub mod algs;
pub mod secret;
pub mod other;

use clap::Parser;
use cli::CliArgs;


fn main() {
    let args = CliArgs::parse();

    if let Some(history) = args.history {
        output::history::show_history(&*history);
        return;
    }

    if let Some(word) = args.diagnose {
        output::diagnose::show_diagnose(word.as_str());
        return;
    }

    if args.karma {
        other::karma::show();
        return;
    }

    if args.daily {
        output::daily::show_daily(args.input.as_deref());
        return;
    }

    if args.stats {
        other::memory::stats();
        return;
    }

    if args.input.is_none() {
        return;
    }

    let (is_secret, idx) = secret::secret::check_word(args.input.clone().unwrap());

    if is_secret {
        secret::secret::give_secret(idx);
        return;
    }

    other::memory::stat_generic(args);
}
