use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Write the default YAML configuration to the configuration directory
    InstallConfiguration,
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
}
