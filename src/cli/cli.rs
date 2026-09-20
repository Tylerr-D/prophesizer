use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about,
    long_about = None
)]
pub struct CliArgs {
    #[arg(
        alias = "input"
    )]
    /// input any word
    pub input: Option<String>,

    #[arg(
        short = 's',
        long = "stats",
    )]
    /// shows stats
    pub stats: bool,

    #[arg(
        short = 'd',
        long = "daily"
    )]
    /// shows daily prophecy
    pub daily: bool,

    #[arg(
        long = "history"
    )]
    /// shows last words you fed to the machine, takes input mode to sort
    ///
    /// modes: 'abc' = alphabetical, 'used' = most used, (anything else) = by latest
    pub history: Option<String>,

    #[arg(
        short = 'g',
        long = "diagnose"
    )]
    /// gives a medical report for a word
    pub diagnose: Option<String>,

    #[arg(
        short = 'k',
        long = "karma"
    )]
    /// shows karma of every word you have fed the machine
    pub karma: bool,
}
