#![deny(warnings)]
#![allow(dead_code)]

use std::error::Error;
use crate::cli::subcommand::{SubCommand};
use crate::toml::config::Config;
use crate::toml::config_file::ConfigFile;

#[derive(Clone, Debug)]
pub struct HierarchicalConfig<'a> {
    config: Config,
    config_file: ConfigFile,
    subcommand: SubCommand<'a>,
}

impl<'a> HierarchicalConfig<'a> {
    pub fn resolve(&self) -> Result<Self, Box<dyn Error>> {
        todo!()
    }
    pub fn config(&self) -> &Config {
        &self.config
    }
}

