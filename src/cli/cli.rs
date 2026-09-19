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
    /// shows the last 10 words you fed to the machine
    pub history: Option<String>,

    #[arg(
        long = "diagnose"
    )]
    /// gives a medical report for a word
    pub diagnose: bool,

    #[arg(
        short = 'k',
        long = "karma"
    )]
    /// shows karma of every word you have fed the machine
    pub karma: bool,

}
