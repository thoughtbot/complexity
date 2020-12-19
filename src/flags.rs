use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Write the default YAML configuration to the configuration directory
    InstallConfiguration,
}

#[derive(Debug)]
pub enum Format {
    Standard,
    Csv,
    Json,
}

impl FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "standard" => Ok(Format::Standard),
            "csv" => Ok(Format::Csv),
            "json" => Ok(Format::Json),
            v => Err(format!("Unknown format: {}", v)),
        }
    }
}

#[derive(Debug)]
pub enum ScoringAlgorithm {
    Standard,
    Length,
}

impl FromStr for ScoringAlgorithm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "standard" => Ok(ScoringAlgorithm::Standard),
            "length" => Ok(ScoringAlgorithm::Length),
            v => Err(format!("Unknown scoring algorithm: {}", v)),
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "complexity",
    about = "A command line tool to identify complex code",
    setting = structopt::clap::AppSettings::ColoredHelp
)]
pub struct Flags {
    /// Ignore files/directories matching the provided value
    ///
    /// This supports providing multiple values with a comma-delimited list
    #[structopt(long, use_delimiter = true)]
    pub ignore: Vec<String>,
    /// Only files/directories matching the provided value
    ///
    /// This supports providing multiple values with a comma-delimited list
    #[structopt(long, use_delimiter = true)]
    pub only: Vec<String>,

    #[structopt(subcommand)]
    pub cmd: Option<Command>,

    /// Format output
    #[structopt(long, possible_values = &["standard", "csv", "json"], default_value = "standard", case_insensitive = true)]
    pub format: Format,

    /// Scoring algorithm
    #[structopt(short = "s", long, possible_values = &["standard", "length"], default_value = "standard", case_insensitive = true)]
    pub scorer: ScoringAlgorithm,
}
