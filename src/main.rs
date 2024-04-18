use std::path::PathBuf;
use ::toml::Value;
use ::toml::Value::Boolean;

use anyhow::Result;
use crate::hierarchical::config::{ Container, Directory, Hadoop };
use clap::{ArgMatches, Args, CommandFactory, Parser, Subcommand};
use serde_derive::{Deserialize, Serialize};
use thiserror::Error;
use toml::config_file::ConfigFile;

use crate::AppError::SubCommandNotPresent;
use crate::cli::subcommand::{parse_kind, SubCommand, SubCommandKind, SubCommandTrait};
use hierarchical::config::Config;
use crate::hierarchical::hierarchical_config::HierarchicalConfig;
use crate::toml::config_file::ConfigFileError;

//use crate::cli::subcommand::SubCommandKind::{ Container, Directory, Hadoop };
use hierarchical::config::ConfigBuilder;
//use crate::Command::Directory;
//use crate::toml::config::{ConfigBuilder, Container};

mod cli;
mod hierarchical;
mod toml;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("the cli subcommand was not present at runtime")]
    SubCommandNotPresent,

    #[error("unknown application error")]
    Unknown,
}

/// This is the entry point for the application.
fn main() -> Result<(), Box<dyn std::error::Error>> {

    // let debug: Value = "true".parse()?;
    // let ccc = Arg {
    //     original_value: debug.to_string(),
    //     original_kind: ArgKind::String,
    //     converted_kind: ArgKind::Bool,
    //     converted_value: debug.try_into()?,
    //};
    //dbg!(ccc);


    match App::command().get_matches().subcommand() {
        None => Err(Box::try_from(SubCommandNotPresent)?),
        Some(sc) => {
            // Get the command line matches for the subcommand that was used.
            // Clap will provide the arguments in priority order (highest to lowest):
            // command line argument, environment variable, or default value.
            let (name, matches) = sc;

            let mut hc = HierarchicalConfig::new(name, matches)?;
            let _config = hc.resolve()?;


            // let app = App::parse();
            // let cp = app.global_opts.config_path;

            Ok(())
        }
    }
}

/// The definition of the command line and its arguments
#[derive(Debug, Parser, Serialize)]
#[command(author, version, about, long_about = None, next_line_help = true, propagate_version = true)]
pub struct App {
    #[command(flatten)]
    global_opts: GlobalOpts,

    #[command(subcommand)]
    command: Command,
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
    //#[strum(serialize = "container", to_string = "container")]
    Container(ContainerCommand),
    //#[strum(serialize = "directory", to_string = "directory")]
    Directory(DirectoryCommand),
    //#[strum(serialize = "hadoop", to_string = "hadoop")]
    Hadoop(HadoopCommand),
}

/// The definition of the command line and its arguments
#[derive(Parser, Debug, Deserialize, Serialize, PartialEq)]
//#[hierarchical]
struct ContainerCommand {
    /// The name of a client
    #[arg(short = 'n', long, env("NAME"), default_value = "kdev")]
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
    #[arg(short = 'p', long, env("PATH"), default_value = ".")]
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
    #[arg(short = 'p', long, env("PATH"), default_value = ".")]
    path: Option<std::path::PathBuf>,
    // /// The connection timeout
    // #[arg(short = 'd', long, value_parser = humantime::parse_duration)]
    // #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    // #[serde(with = "humantime_serde")]
    // connection_timeout: Option<std::time::Duration>,
}
