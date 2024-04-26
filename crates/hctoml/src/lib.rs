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
pub struct TomlFile {
    file_data: HashMap<String, Value>,
}

impl TomlFile {
    #[inline]
    pub fn new(config_path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let file_data = Self::parse_file(config_path)?;
        Ok(Self { file_data })
    }

    #[inline]
    pub fn try_get_value_from_table(
        &self,
        table_name: &str,
        field_name: &str,
    ) -> Result<Option<&Value>, Box<dyn Error>> {
        match self
            .file_data
            .get(table_name)
            .and_then(|x| x.get(field_name))
        {
            None => Err(Box::from(FieldNotFound {
                table_name: Some(table_name.to_owned()),
                field_name: field_name.to_owned(),
            })),
            Some(value) => Ok(Some(value)),
        }
    }

    #[inline]
    pub fn try_get_value(&self, field_name: &str) -> Result<Option<String>, Box<dyn Error>> {
        match self.file_data.get(field_name) {
            None => Err(Box::from(FieldNotFound {
                table_name: None,
                field_name: field_name.to_owned(),
            })),
            Some(value) => Ok(Some(value.to_string())),
        }
    }

    fn parse_file(
        config_path: &PathBuf,
    ) -> Result<HashMap<String, Value>, Box<dyn Error>> {
        let content = fs::read_to_string(config_path)?;
        Ok(toml::from_str::<HashMap<String, Value>>(
            content.as_str(),
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
