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
    /// shows history of previous inputs
    pub history: Option<String>,
}
