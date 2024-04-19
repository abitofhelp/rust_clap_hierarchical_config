#![deny(warnings)]
#![allow(dead_code)]

use std::path::PathBuf;
use std::str::FromStr;

use clap::ArgMatches;

use crate::cli::subcommand::SubCommand;
use crate::cli::subcommand_kind::{parse_kind, SubCommandKind};
use crate::hierarchical::config::{Config, ConfigBuilder, Container, Directory, Hadoop};
use crate::toml::config_file::ConfigFile;

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

        // We want to have hierarchical argument values with the following priorities (highest to lowest):
        // cli > envvar > toml file > defaults.
        // So, we need to determine the matches have default values.  If we have a value in the
        // config file, we will use it.  Otherwise, the default value is retained.
        let default_ids = subcommand.try_get_default_value_matches(vec!["config_path"])?;

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

        // TODO:
        //for id in default_ids.iter().map(|x| x.as_str()) {}

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



        Ok(HierarchicalConfig { config })
    }

    // fn resolve_arg_value<T>(
    //     &self,
    //     name: &str,
    //     subcommand: &SubCommand,
    //     config_file: &ConfigFile,
    //     default_ids: &Vec<String>) -> Result<T, Box<dyn std::error::Error>> {
    //
    //     for id in default_ids.iter().map(|x| x.as_str()) {
    //         // Fetch the value of the field/id from the toml configuration data.
    //         let config_value = config_file
    //             .try_get_value_from_table(subcommand.name(), id)?
    //             .map(|x| x.to_string());
    //
    //         // Fetch the value of the field/id from the subcommand's matches.
    //         let match_value = subcommand.try_get_one_arg::<String>(id)?;
    //
    //         // Determine whether to update the field in the Config object.
    //         if config_value != match_value {
    //             return Ok(config_value);
    //         } else {
    //             return Ok(match_value);
    //         }
    //     }
    // }

    pub fn config(&self) -> &Config {
        &self.config
    }
}
