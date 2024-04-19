#![deny(warnings)]
#![allow(dead_code)]

use crate::cli::subcommand::{parse_kind, SubCommand, SubCommandKind, SubCommandTrait};
use crate::hierarchical::config::{Config, ConfigBuilder, Container, Directory, Hadoop};
use crate::toml::config_file::{ConfigFile};
use clap::ArgMatches;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct HierarchicalConfig {
    config: Config,
}

impl<'a> HierarchicalConfig {
    pub(crate) fn new(
        name: &'a str,
        matches: &'a ArgMatches,
    ) -> anyhow::Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let subcommand = SubCommand::new(name, matches)?;
        let subcommand_kind = parse_kind(subcommand.name()).unwrap();

        // Get the path to the .toml configuration file, which may not exist.
        // The path to the .toml configuration file CANNOT be set in the toml file, only on the
        // command line.
        let config_path = subcommand
            .try_get_one_arg::<PathBuf>("config_path")?
            .unwrap();

        // Parse the .toml configuration file, so we can determine values for substitution in
        // the command line matches.
        let config_file = ConfigFile::new(&config_path)?;

        // Initialize configuration values that are not in toml tables; Hence, used for
        // any kind of subcommand.  Most likely, these values will exist in the toml
        // config file, but not on the command line (i.e. debug in this example app).
        let debug = config_file
            .try_get_value("debug")?
            .map(|x| bool::from_str(x.as_str()))
            .transpose()?;

        // Build a mutable Config from the command line matches.  It will be updated
        // when default values vs hierarchical file values are determined.
        let config = match subcommand_kind {
            SubCommandKind::Container => {
                // Initialize an empty config instance.
                let mut config = Container::default();

                // Load the matches into the Config instance.
                config.name = subcommand.try_get_one_arg::<String>("name")?;

                ConfigBuilder::default()
                    .debug(debug)
                    .container(config)
                    .directory(None)
                    .hadoop(None)
                    .build()?
            }
            SubCommandKind::Directory => {
                // Initialize an empty config instance.
                let mut config = Directory::default();

                // Load the matches into the Config instance.
                config.path = subcommand.try_get_one_arg::<PathBuf>("path")?;

                ConfigBuilder::default()
                    .debug(debug)
                    .container(None)
                    .directory(config)
                    .hadoop(None)
                    .build()?
            }
            SubCommandKind::Hadoop => {
                // Initialize an empty config instance.
                let mut config = Hadoop::default();

                // Load the matches into the Config instance.
                config.path = subcommand.try_get_one_arg::<PathBuf>("path")?;

                ConfigBuilder::default()
                    .debug(debug)
                    .container(None)
                    .directory(None)
                    .hadoop(config)
                    .build()?
            }
        };

        // We want to have hierarchical argument values with the following priorities (highest to lowest):
        // cli > envvar > toml file > defaults.
        // So, we need to determine the matches have default values.  If we have a value in the
        // config file, we will use it.  Otherwise, the default value is retained.
        let default_ids = subcommand.try_get_default_value_matches(vec!["config_path"])?;

        for id in default_ids.iter().map(|x| x.as_str()) {
            // Fetch the value of the field/id from the toml configuration data.
            let config_value = config_file
                .try_get_value_from_table(subcommand.name(), id)?
                .map(|x| x.to_string());

            // Fetch the value of the field/id from the subcommand's matches.
            let match_value = subcommand.try_get_one_arg::<String>(id)?;

            // Determine whether to update the field in the Config object.
            if config_value.clone() != match_value {
                // Use the config_value rather than the default that Clap supplied.
                match subcommand_kind {
                    SubCommandKind::Container => {
                        let mut container = config.container.clone().unwrap();
                        container.name = Some(config_value.clone().unwrap().to_string())
                    }
                    SubCommandKind::Directory => {
                        let mut directory = config.directory.clone().unwrap();
                        directory.path = Some(PathBuf::from(config_value.clone().unwrap().to_string()))
                    }
                    SubCommandKind::Hadoop => {
                        let mut hadoop = config.hadoop.clone().unwrap();
                        hadoop.path = Some(PathBuf::from(config_value.clone().unwrap().to_string()))
                    }
                };
                println!("Setting id '{id}' to use toml value '{}' rather than Clap's default value '{}'", config_value.unwrap(), match_value.unwrap())
            } else {
                println!(
                    "Retaining id '{id}' use of Clap's default '{}'",
                    config_value.unwrap()
                )
            }
        }

        Ok(HierarchicalConfig { config })
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}
