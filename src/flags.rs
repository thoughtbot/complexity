use clap::{Parser, Subcommand, ValueEnum};
use std::str::FromStr;

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Write the default YAML configuration to the configuration directory
    InstallConfiguration,
}

#[derive(Debug, Clone, ValueEnum)]
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

#[derive(Debug, Clone, ValueEnum)]
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

#[derive(Debug, Parser)]
#[clap(name = "complexity")]
#[clap(about = "A command line tool to identify complex code", long_about = None)]
pub struct Flags {
    /// Ignore files/directories matching the provided value
    ///
    /// This supports providing multiple values with a comma-delimited list
    #[clap(long, value_delimiter = ',')]
    pub ignore: Vec<String>,
    /// Only files/directories matching the provided value
    ///
    /// This supports providing multiple values with a comma-delimited list
    #[clap(long, value_delimiter = ',')]
    pub only: Vec<String>,

    #[clap(subcommand)]
    pub cmd: Option<Command>,

    /// Format output
    #[clap(long, value_parser, default_value = "standard")]
    pub format: Format,

    /// Scoring algorithm
    #[clap(short, long, value_parser, default_value = "standard")]
    pub scorer: ScoringAlgorithm,
}
