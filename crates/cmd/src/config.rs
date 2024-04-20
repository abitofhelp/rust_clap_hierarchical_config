#![deny(warnings)]
#![allow(dead_code)]

use std::error::Error;
use std::path::PathBuf;
use std::str::FromStr;

use clap::ArgMatches;
use toml::value::Value;
use derive_builder::Builder;
use serde_derive::Deserialize;
use hctoml::config_file::ConfigFile;

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

        Ok(Config { config })
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
            SubCommandKind::Container => ConfigBuilder::default()
                .debug(debug)
                .container(Self::build_container_config(
                    &subcommand,
                    &default_ids,
                    &config_file,
                )?)
                .directory(None)
                .hadoop(None)
                .build()?,
            SubCommandKind::Directory => ConfigBuilder::default()
                .debug(debug)
                .container(None)
                .directory(Self::build_directory_config(
                    &subcommand,
                    &default_ids,
                    &config_file,
                )?)
                .hadoop(None)
                .build()?,
            SubCommandKind::Hadoop => ConfigBuilder::default()
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

    pub fn config(&self) -> &Config {
        &self.config
    }
}