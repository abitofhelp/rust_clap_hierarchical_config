#![deny(warnings)]
#![allow(dead_code)]

use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use toml::Value;

use crate::error::ConfigFileError::FieldNotFound;

mod error;

#[derive(Clone, Debug)]
pub struct ConfigFile {
    config_file_data: HashMap<String, Value>,
}

impl ConfigFile {
    pub fn new(config_path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let config_file_data = Self::parse_config_file_data(config_path)?;
        Ok(Self { config_file_data })
    }

    pub fn try_get_value_from_table(
        &self,
        table_name: &str,
        field_name: &str,
    ) -> Result<Option<&Value>, Box<dyn Error>> {
        match self
            .config_file_data
            .get(table_name)
            .and_then(|x| x.get(field_name))
        {
            None => Err(Box::try_from(FieldNotFound {
                table_name: Some(table_name.to_string()),
                field_name: field_name.to_string(),
            })?),
            Some(v) => Ok(Some(v)),
        }
    }

    pub fn try_get_value(&self, field_name: &str) -> Result<Option<String>, Box<dyn Error>> {
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
    ) -> Result<HashMap<String, Value>, Box<dyn Error>> {
        let content_string = fs::read_to_string(config_path)?;
        Ok(toml::from_str::<HashMap<String, Value>>(
            content_string.as_str(),
        )?)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         assert_eq!(true, true);
//     }
// }
