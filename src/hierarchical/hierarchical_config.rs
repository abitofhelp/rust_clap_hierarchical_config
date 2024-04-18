#![deny(warnings)]
#![allow(dead_code)]

use crate::cli::subcommand::{parse_kind, SubCommand, SubCommandKind, SubCommandTrait};
use crate::hierarchical::config::{Config, ConfigBuilder, Container, Directory, Hadoop};
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


        // Set the values in the config instance for non-table values in the toml config file.
        // i.e. debug is just a field and is not under a table name, indicated by [tablename_here].

        // Set Clap's command line matches into the config instance...

        // Set the toml config table values into the config instance...
        // Overwrite any values that exist in the config file but Clap marked as coming from
        // a default source.



        // Build a mutable Config from the command line matches.  It will be updated
        // when default values vs hierarchical file values are determined.
        let config = match parse_kind(subcommand.name()).unwrap() {
            SubCommandKind::Container => {
                // Initialize an empty config instance.
                let mut config = Container::default();


                config.name = subcommand.get_one_arg::<String>("name")?;

                ConfigBuilder::default()
                    .debug(None)
                    .container(config)
                    .directory(None)
                    .hadoop(None)
                    .build()?
            },
            SubCommandKind::Directory => {
                // Initialize an empty config instance.
                let mut config = Directory::default();

                config.path = subcommand.get_one_arg::<PathBuf>("path")?;

                ConfigBuilder::default()
                    .debug(None)
                    .container(None)
                    .directory(config)
                    .hadoop(None)
                    .build()?
            },
            SubCommandKind::Hadoop => {
                // Initialize an empty config instance.
                let mut config = Hadoop::default();

                config.path = subcommand.get_one_arg::<PathBuf>("path")?;

                ConfigBuilder::default()
                    .debug(None)
                    .container(None)
                    .directory(None)
                    .hadoop(config)
                    .build()?
            },
        };

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

    pub fn resolve(&mut self) -> Result<Config, Box<dyn Error>> {
        // For any execution of the application, only a single subcommand will be active.

        // Initialize configuration values that are not in toml tables; Hence, used for
        // any kind of subcommand.  Most likely, these values will exist in the toml
        // config file, but not on the command line (i.e. debug in this example app).
        let debug = self
            .config_file
            .get_value("debug")?
            .map(|x| bool::from_str(x.as_str()))
            .transpose()?;

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
            let match_value: &str = self.subcommand.get_one_arg::<&str>(id)?.ok_or(
                // FIXME: Wrong error, should be for subcommands not config files.
                ConfigFileError::NotFound {
                    path: String::from("name"),
                },
            )?;

            let config = match parse_kind(self.subcommand.name()).unwrap() {
                SubCommandKind::Container => {
                    let mut config = Container::default();
                    config.name = self.subcommand.get_one_arg::<String>("name")?;

                    ConfigBuilder::default()
                        .debug(debug)
                        .container(config)
                        .directory(None)
                        .hadoop(None)
                        .build()?
                },
                SubCommandKind::Directory => {
                    let mut config = Directory::default();
                    config.path = self.subcommand.get_one_arg::<PathBuf>("path")?;

                    ConfigBuilder::default()
                        .debug(debug)
                        .container(None)
                        .directory(config)
                        .hadoop(None)
                        .build()?
                },
                SubCommandKind::Hadoop => {
                    let mut config = Hadoop::default();
                    config.path = self.subcommand.get_one_arg::<PathBuf>("path")?;

                    ConfigBuilder::default()
                        .debug(debug)
                        .container(None)
                        .directory(None)
                        .hadoop(config)
                        .build()?
                },
            };

            // // Initialize the Config using subcommand (Clap) matches.
            // // Clap provides hierarchical argument values:
            // // cli > envvar > defaults.  Since it does not provide configuration file
            // // configuration values, we need to integrate it into the final Config.
            // let config = match parse_kind(self.subcommand.name()).unwrap() {
            //     SubCommandKind::Container => {
            //
            //     },
            //     SubCommandKind::Directory => {
            //
            //     },
            //     SubCommandKind::Hadoop => {
            //
            //     },
            // };


            // Determine whether to update the field in the Config object.
            if config_value.is_some_and(|x| x.as_str() != Option::from(match_value)) {
                // Use the config_value rather than the default that Clap supplied.
                match self.subcommand.name() {
                    "container" => {
                        config.container.unwrap().name = Some(config_value.unwrap().to_string())
                    }
                    "directory" => {
                        config.directory.unwrap().path =
                            Some(PathBuf::from(config_value.unwrap().to_string()))
                    }
                    "hadoop" => {
                        config.hadoop.unwrap().path =
                            Some(PathBuf::from(config_value.unwrap().to_string()))
                    }
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
