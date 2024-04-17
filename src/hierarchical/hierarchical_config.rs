#![deny(warnings)]
#![allow(dead_code)]

use crate::cli::subcommand::{SubCommand, SubCommandTrait};
use crate::toml::config::Config;
use crate::toml::config_file::{ConfigFile, ConfigFileError};
use clap::ArgMatches;
use std::error::Error;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct HierarchicalConfig<'a> {
    config: Config,
    config_file: ConfigFile,
    subcommand: SubCommand<'a>,
}

impl<'a> HierarchicalConfig<'a> {
    pub(crate) fn new(
        name: &'a str,
        matches: &'a ArgMatches,
    ) -> anyhow::Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let subcommand = SubCommand::new(name, matches)?;

        // Build a mutable Config from the command line matches.  It will be updated
        // when default values vs hierarchical file values are determined.
        let config = Config::new();

        // Get the path to the .toml configuration file, which may not exist.
        // The path to the .toml configuration file CANNOT be set in the toml file, only on the
        // command line.
        let config_path =
            subcommand
                .get_one_arg::<PathBuf>("config_path")?
                .ok_or(ConfigFileError::NotFound {
                    path: String::from("config_path"),
                })?;
        // Parse the .toml configuration file, so we can determine values for substitution in
        // the command line matches.
        let config_file = ConfigFile::new(&config_path)?;

        Ok(HierarchicalConfig {
            config,
            config_file,
            subcommand,
        })
    }

    fn initialize_common_config(&mut self) -> Result<(), Box<dyn Error>> {
        // Load all the values that were provided by matches in subcommand.
        self.config.debug = self
            .config_file
            .get_value("debug")?
            .map(|x| bool::from_str(x.as_str()))
            .transpose()?;
        Ok(())
    }

    fn initialize_container_config(&mut self) -> Result<(), Box<dyn Error>> {
        // Load all the values that were provided by matches in subcommand.
        self.config.container.name = self.subcommand.get_one_arg::<String>("name")?;
        Ok(())
    }

    fn initialize_directory_config(&mut self) -> Result<(), Box<dyn Error>> {
        // Load all the values that were provided by matches in subcommand.
        self.config.directory.path = self.subcommand.get_one_arg::<PathBuf>("path")?;
        Ok(())
    }

    fn initialize_hadoop_config(&mut self) -> Result<(), Box<dyn Error>> {
        // Load all the values that were provided by matches in subcommand.
        self.config.hadoop.path = self.subcommand.get_one_arg::<PathBuf>("path")?;
        Ok(())
    }

    pub fn resolve(&mut self) -> Result<Config, Box<dyn Error>> {
        // For any execution of the application, only a single subcommand will be active.

        // Initialize the Config using subcommand (Clap) matches.
        // Clap provides hierarchical argument values:
        // cli > envvar > defaults.  Since it does not provide configuration file
        // configuration values, we need to integrate it into the final Config.
        match self.subcommand.name() {
            "container" => self.initialize_container_config()?,
            "directory" => self.initialize_directory_config()?,
            "hadoop" => self.initialize_hadoop_config()?,
            _ => {
                // FIXME: Wrong error, should be for subcommands not config files.
                return Err(Box::try_from(ConfigFileError::NotFound {
                    path: String::from("path"),
                })
                .unwrap());
            }
        };

        // Initialize configuration values that are not in toml tables; Hence, used for
        // any kind of subcommand.  Most likely, these values will exist in the toml
        // config file, but not on the command line (i.e. debug in this example app).
        self.initialize_common_config()?;

        // Finally, we want to have a hierarchical argument values with the following priority:
        // cli > envvar > toml file > defaults.
        // So, we will determine the arguments for the current subcommand that have default values.
        // If we have a value in the config file, we will use it.  Otherwise, the default value is
        // retained.
        let default_ids = self
            .subcommand
            .get_default_value_matches(vec!["config_path"])?;
        for id in default_ids.iter() {
            // Fetch the value of the field/id from the toml configuration data.
            let config_value = self
                .config_file
                .get_value_from_table(self.subcommand.name(), id.as_str())?;

            // Fetch the value of the field/id from the subcommand's matches.
            let match_value: String = self.subcommand.get_one_arg::<String>(id)?.ok_or(
                // FIXME: Wrong error, should be for subcommands not config files.
                ConfigFileError::NotFound {
                    path: String::from("name"),
                },
            )?;

            // Determine whether to update the field in the Config object.
            if config_value.clone().is_some_and(|x| x != *match_value.clone()) {
                // Use the config_value rather than the default that Clap supplied.
                match self.subcommand.name() {
                    "container" => self.config.container.name = config_value.clone(),
                    "directory" => self.config.directory.path = Some(PathBuf::from(match_value.clone())),
                    "hadoop" => self.config.hadoop.path = Some(PathBuf::from(match_value.clone())),
                    _ => {
                        // FIXME: Wrong error, should be for subcommands not config files.
                        return Err(Box::try_from(ConfigFileError::NotFound {
                            path: String::from("path"),
                        })
                        .unwrap());
                    }
                };
                println!("Setting id '{id}' to use toml value '{}' rather than Clap's default value '{match_value}'", config_value.unwrap())
            } else {
                println!("Retaining id '{id}' use of Clap's default '{match_value}'")
            }
        }
        Ok(self.config.clone())
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}
