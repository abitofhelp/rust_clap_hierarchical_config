use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, CommandFactory, Parser, Subcommand};
use serde_derive::{Deserialize, Serialize};
use thiserror::Error;
use toml::config_file::ConfigFile;

use crate::AppError::SubCommandNotPresent;
use crate::cli::subcommand::{SubCommand, SubCommandTrait};
use toml::config::Config;
use crate::toml::config_file::ConfigFileError;

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
    match App::command().get_matches().subcommand() {
        None => Err(Box::try_from(SubCommandNotPresent)?),
        Some(sc) => {
            // Get the command line matches for the subcommand that was used.
            // Clap will provide the arguments in priority order (highest to lowest):
            // command line argument, environment variable, or default value.
            let (name, matches) = sc;
            let subcommand = SubCommand::new(name, matches)?;

            // Build a mutable Config from the command line matches.  It will be updated
            // when default values vs hierarchical file values are determined.
            let mut config = Config::new();
            config.debug = Some(true);
            config.container.name = subcommand.get_one_arg::<String>("name")?;

            // ONLY ARGMATCHES ARE SET HERE...
            // match _subcommand_name {
            //     "container" =>  { hierarchical.container.clone().unwrap().name = subcommand_matches.get_one::<String>("name").cloned() }
            //     "directory" => { hierarchical.directory.clone().unwrap().path = subcommand_matches.get_one::<PathBuf>("path").cloned() }
            //     "hadoop" => { hierarchical.hadoop.clone().unwrap().path = subcommand_matches.get_one::<PathBuf>("path").cloned() }
            //     &_ => { return  Err(Box::try_from(AppError::SubCommandNotPresent).unwrap()) }
            // }

            // Scan the matches looking for any that are from the 'default' source.
            // Our goal is to have a hierarchical determination of configuration settings
            // in priority order (highest to lowest):
            // command line argument, environment variable, toml file, or default value.
            // So, we need to determine arguments that have default values and determine whether
            // they need to be set with values from the toml file, which has a higher priority than
            // default values.
            let default_ids = subcommand.get_default_value_matches(vec!["config_path"]);

            // Get the path to the .toml configuration file, which may not exist.
            // The path to the .toml configuration file CANNOT be set in the toml file, only on the
            // command line.
            let config_path = subcommand.get_one_arg::<PathBuf>("config_path")?.ok_or(
                ConfigFileError::NotFound {
                    path: String::from("config_path"),
                },
            )?;
            // Parse the .toml configuration file, so we can determine values for substitution in
            // the command line matches.
            let config_file = ConfigFile::new(&config_path)?;

            // for id in default_ids.iter().copied() {
            //     let config_value = config_file.get_value_from_table(_subcommand_name, id)?;
            //     let match_value= subcommand_matches.get_one::<String>(id).ok_or(
            //         TomlConfigFileError::NotFound {
            //             path: String::from("name"),
            //         },
            //     )?;

            // if config_value.clone().is_some_and(|x| x != *match_value) {
            //     // Use the config_value rather than the default that Clap supplied.
            //     // Directory:
            //     hierarchical.container.clone().unwrap().name = config_value.clone();
            //     println!("Setting id '{id}' to use hierarchical '{}' rather than Clap's default '{match_value}'", config_value.unwrap())
            // } else {
            //     println!("Retaining id '{id}' use of Clap's default '{match_value}'")
            // }
            //}

            // Merge argument matches with updates into a Config instance.

            //{
            //    dbg!(id);
            //}

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
    #[arg(short = 'c', long, global = true, default_value = "hierarchical.toml")]
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
