use anyhow::{Result};
use clap::{Args, CommandFactory, Parser, Subcommand};
use serde_derive::{Deserialize, Serialize};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("the cli subcommand was not present at runtime")]
    SubCommandNotPresent,

    #[error("unknown application error")]
    Unknown,
}

// #![deny(warnings)]
// #![allow(dead_code)]
use hctoml::ConfigFile;
use subcommand::{ SubCommand };
use subcommand::kind::Kind;
use subcommand::kind::parse_kind;

use std::error::Error;
use std::path::PathBuf;
use std::str::FromStr;

use clap::ArgMatches;
use toml::value::Value;
use derive_builder::Builder;
//use serde_derive::Deserialize;
//use toml::config_file::ConfigFile;

#[derive(Builder, Clone, Debug, Default, Deserialize)]
#[builder(setter(into))]
pub struct Config {
    pub debug: Option<bool>,
    pub container: Option<Container>,
    pub directory: Option<Directory>,
    pub hadoop: Option<Hadoop>,
}

#[derive(Builder, Clone, Debug, Default, Deserialize)]
#[builder(setter(into))]
pub struct Container {
    pub name: Option<String>,
}

#[derive(Builder, Clone, Debug, Default, Deserialize)]
#[builder(setter(into))]
pub struct Directory {
    pub path: Option<PathBuf>,
}

#[derive(Builder, Clone, Debug, Default, Deserialize)]
#[builder(setter(into))]
pub struct Hadoop {
    pub path: Option<PathBuf>,
}

impl<'a> Config {
    pub(crate) fn new(
        name: &'a str,
        matches: &'a ArgMatches,
    ) -> Result<Self, Box<dyn std::error::Error>>
        where
            Self: Sized,
    {
        let subcommand = SubCommand::new(name, matches)?;

        // We want to have hierarchical argument values with the following priorities (highest to lowest):
        // cli > envvar > hctoml file > defaults.
        // So, we need to determine the matches have default values.  If we have a value in the
        // config file, we will use it.  Otherwise, the default value is retained.
        let default_ids = subcommand.try_get_default_value_matches(vec!["config_path"])?;

        // Get the path to the .hctoml configuration file, which may not exist.
        // The path to the .hctoml configuration file CANNOT be set in the hctoml file, only on the
        // command line.
        let config_file = Self::parse_config_file(&subcommand)?;

        // Initialize configuration values that are not in hctoml tables; Hence, used for
        // any kind of subcommand.  Most likely, these values will exist in the hctoml
        // config file, but not on the command line (i.e. debug in this example app).
        let debug = config_file
            .try_get_value("debug")?
            .map(|x| bool::from_str(x.as_str()))
            .transpose()?;

        // Build a mutable Config from the command line matches.  It will be updated
        // when default values vs hierarchical file values are determined.
        let config = Self::build_config(
            &subcommand,
            &default_ids,
            &config_file,
            debug,
        )?;

        Ok(config)
    }

    fn build_config(
        subcommand: &SubCommand,
        default_ids: &Vec<String>,
        config_file: &ConfigFile,
        debug: Option<bool>,
    ) -> Result<Config, Box<dyn Error>> {
        // Build a mutable Config from the command line matches.  It will be updated
        // when default values vs hierarchical file values are determined.

        let subcommand_kind = parse_kind(subcommand.name()).unwrap();

        let config = match subcommand_kind {
            Kind::Container => ConfigBuilder::default()
                .debug(debug)
                .container(Self::build_container_config(
                    &subcommand,
                    &default_ids,
                    &config_file,
                )?)
                .directory(None)
                .hadoop(None)
                .build()?,
            Kind::Directory => ConfigBuilder::default()
                .debug(debug)
                .container(None)
                .directory(Self::build_directory_config(
                    &subcommand,
                    &default_ids,
                    &config_file,
                )?)
                .hadoop(None)
                .build()?,
            Kind::Hadoop => ConfigBuilder::default()
                .debug(debug)
                .container(None)
                .directory(None)
                .hadoop(Self::build_hadoop_config(
                    &subcommand,
                    &default_ids,
                    &config_file,
                )?)
                .build()?,
        };
        Ok(config)
    }

    fn parse_config_file(subcommand: &SubCommand) -> Result<ConfigFile, Box<dyn Error>> {
        // Get the path to the .hctoml configuration file, which may not exist.
        // The path to the .hctoml configuration file CANNOT be set in the hctoml file, only on the
        // command line.
        let config_path = subcommand
            .try_get_one_arg::<PathBuf>("config_path")?
            .unwrap();

        // Parse the .hctoml configuration file, so we can determine values for substitution in
        // the command line matches.
        let config_file = ConfigFile::new(&config_path)?;

        Ok(config_file)
    }

    fn build_hadoop_config(
        subcommand: &SubCommand,
        default_ids: &Vec<String>,
        config_file: &ConfigFile,
    ) -> Result<Hadoop, Box<dyn Error>> {
        // Initialize an empty config instance.
        let mut config = Hadoop::default();

        // Load the matches into the Config instance.
        config.path =
            Self::resolve_arg_value::<PathBuf>("path", &default_ids, &config_file, &subcommand)?
                .map(|x| PathBuf::from(x.to_string()));
        Ok(config)
    }

    fn build_directory_config(
        subcommand: &SubCommand,
        default_ids: &Vec<String>,
        config_file: &ConfigFile,
    ) -> Result<Directory, Box<dyn Error>> {
        // Initialize an empty config instance.
        let mut config = Directory::default();

        // Load the matches into the Config instance.
        config.path =
            Self::resolve_arg_value::<PathBuf>("path", &default_ids, &config_file, &subcommand)?
                .map(|x| PathBuf::from(x.to_string()));

        Ok(config)
    }

    fn build_container_config(
        subcommand: &SubCommand,
        default_ids: &Vec<String>,
        config_file: &ConfigFile,
    ) -> Result<Container, Box<dyn Error>> {
        // Initialize an empty config instance.
        let mut config = Container::default();

        // Update any values that Clap used from default sources,
        // if the argument exists in the hctoml config file.
        // Load the matches into the Config instance.

        config.name =
            Self::resolve_arg_value::<String>("name", &default_ids, &config_file, &subcommand)?
                .map(|x| x.to_string());

        Ok(config)
    }

    fn resolve_arg_value<T: Clone + Send + Sync + 'static + serde::Serialize>(
        id: &str,
        default_values: &Vec<String>,
        config_file: &ConfigFile,
        subcommand: &SubCommand,
    ) -> Result<Option<Value>, Box<dyn Error>> {
        if default_values.contains(&id.to_string()) {
            // Fetch the value of the field/id from the hctoml configuration data.
            let config_value = config_file
                .try_get_value_from_table(subcommand.name(), id)?
                .cloned();

            // Fetch the value of the field/id from the subcommand's matches.
            let match_value = subcommand.try_get_one_arg::<T>(id)?;
            let mv = Some(Value::try_from(match_value)?);

            // Determine whether to use the value from the config file or retain the default value.
            let value = if config_value != mv { config_value } else { mv };
            Ok(value)
        } else {
            // Use the value that Clap determined because its source is higher priority
            // than a config file or default value.
            let default_value = subcommand.try_get_one_arg::<T>(id)?;
            Ok(Some(Value::try_from(default_value)?))
        }
    }
}

/// This is the entry point for the application.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    match App::command().get_matches().subcommand() {
        None => Err(Box::try_from(AppError::SubCommandNotPresent)?),
        Some(sc) => {
            // Get the command line matches for the subcommand that was used.
            // Clap will provide the arguments in priority order (highest to lowest):
            // command line argument, environment variable, or default value.
            let (name, matches) = sc;
            let hc = Config::new(name, matches)?;
            dbg!(hc);


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
    /// config_path is the path to a configuration (.hctoml) file, which defaults to the current directory.
    #[arg(short = 'c', long, global = true, default_value = "config.hctoml")]
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
