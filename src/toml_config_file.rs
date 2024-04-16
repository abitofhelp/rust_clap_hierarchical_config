#![deny(warnings)]
#![allow(dead_code)]

use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use thiserror::Error;
use crate::toml_config_file::TomlConfigFileError::FieldNotFound;

#[derive(Error, Debug)]
pub enum TomlConfigFileError {
    #[error("the field '{field_name:?}' was not found in the toml configuration file table '{table_name:?}'")]
    FieldNotFound {
        table_name: Option<String>,
        field_name: String,
    },

    #[error("the toml configuration file '{path:?}' was not found")]
    NotFound { path: String },

    #[error("unknown toml configuration file error")]
    Unknown,
}

pub struct TomlConfigFile {
    config_file_data: HashMap<String, toml::Value>,
}

impl TomlConfigFile {
    pub fn new(config_file: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let config_file_data = Self::parse_config_file_data(config_file)?;
        Ok(Self { config_file_data })
    }

    pub(crate) fn get_value_from_table(
        &self,
        table_name: &str,
        field_name: &str,
    ) -> Result<Option<String>, Box<dyn Error>> {
        match self
            .config_file_data
            .get(table_name)
            .unwrap()
            .get(field_name)
        {
            None => Err(Box::try_from(FieldNotFound {
                table_name: Some(table_name.to_string()),
                field_name: field_name.to_string(),
            })?),
            Some(v) => Ok(Some(v.to_string())),
        }
    }

    pub(crate) fn get_value(&self, field_name: &str) -> Result<Option<String>, Box<dyn Error>> {
        match self.config_file_data.get(field_name) {
            None => Err(Box::try_from(FieldNotFound {
                table_name: None,
                field_name: field_name.to_string(),
            })?),
            Some(v) => Ok(Some(v.to_string())),
        }
    }

    fn parse_config_file_data(
        config_path: &PathBuf,
    ) -> Result<HashMap<String, toml::Value>, Box<dyn std::error::Error>> {
        let content_string = fs::read_to_string(config_path)?;
        Ok(toml::from_str::<HashMap<String, toml::Value>>(
            content_string.as_str(),
        )?)
    }
}
