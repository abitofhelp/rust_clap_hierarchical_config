use std::error::Error;
use std::str::FromStr;

use clap::{ArgMatches, Args, CommandFactory, Parser, Subcommand};
use serde_derive::{Deserialize, Serialize};

use subcommand::kind::Kind;

use crate::error::AppError;

/// The definition of the command line and its arguments
#[derive(Debug, Parser, Serialize)]
#[command(author, version, about, long_about = None, next_line_help = true, propagate_version = true)]
pub struct App {
    #[command(flatten)]
    global_opts: GlobalOpts,

    #[command(subcommand)]
    command: Command,
}

impl App {
    pub fn subcommand() -> Result<(Kind, ArgMatches), Box<dyn Error>> {
        match App::command().get_matches().subcommand() {
            None => Err(Box::from(AppError::SubCommandNotPresent)),
            Some(sc) => {
                // Get the command line matches for the subcommand that was used.
                // Clap will provide the arguments in priority order (highest to lowest):
                // command line argument, environment variable, or default value.
                let (subcommand_name, matches) = sc;
                let kind = Kind::from_str(subcommand_name)?;
                Ok((kind, matches.clone()))
            }
        }
    }
}

#[derive(Debug, Args, Serialize)]
struct GlobalOpts {
    /// config_path is the path to a configuration (.toml) file, which defaults to the current directory.
    #[arg(short = 'c', long, global = true, default_value = "config.toml")]
    config_path: std::path::PathBuf,
}

#[derive(
Debug,
Subcommand,
Deserialize,
Serialize,
PartialEq, //, EnumString, strum_macros::Display,
)]
enum Command {
    #[command(about = "The hash container command determines the base64 binary MD5 hash for each blob in a container.", long_about = None)]
    /// Determines the base64 binary MD5 hash for each blob in a container
    Container(ContainerCommand),
    Directory(DirectoryCommand),
    Hadoop(HadoopCommand),
}

/// The definition of the command line and its arguments
#[derive(Parser, Debug, Deserialize, Serialize, PartialEq)]
//#[hierarchical]
struct ContainerCommand {
    /// The name of a client
    #[arg(short = 'n', long, env("HCCC_NAME"), default_value = "kdev")]
    name: Option<String>,
    // /// The connection timeout
    // #[arg(short = 'd', long, value_parser = humantime::parse_duration)]
    // #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    // #[serde(with = "humantime_serde")]
    // connection_timeout: Option<std::time::Duration>,
}

/// The definition of the command line and its arguments
#[derive(Parser, Debug, Deserialize, Serialize, PartialEq)]
//#[hierarchical]
struct DirectoryCommand {
    /// path is the path to the directory on the file system, which defaults to the current directory.
    #[arg(short = 'p', long, env("HCDC_PATH"), default_value = ".")]
    path: Option<std::path::PathBuf>,
    // /// The connection timeout
    // #[arg(short = 'd', long, value_parser = humantime::parse_duration)]
    // #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    // #[serde(with = "humantime_serde")]
    // connection_timeout: Option<std::time::Duration>,
}

/// The definition of the command line and its arguments
#[derive(Parser, Debug, Deserialize, Serialize, PartialEq)]
//#[hierarchical]
struct HadoopCommand {
    /// path is the path to the directory on the file system, which defaults to the current directory.
    #[arg(short = 'p', long, env("HCHC_PATH"), default_value = ".")]
    path: Option<std::path::PathBuf>,
    // /// The connection timeout
    // #[arg(short = 'd', long, value_parser = humantime::parse_duration)]
    // #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    // #[serde(with = "humantime_serde")]
    // connection_timeout: Option<std::time::Duration>,
}
