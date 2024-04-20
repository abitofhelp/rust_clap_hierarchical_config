#![deny(warnings)]
#![allow(dead_code)]

use std::any::Any;
use std::error::Error;

use clap::ArgMatches;
use clap::parser::ValueSource;

pub mod kind;

#[derive(Clone, Debug)]
pub struct SubCommand<'a> {
    name: &'a str,
    matches: &'a ArgMatches,
}

impl<'a> SubCommand<'a> {
    pub fn new(name: &'a str, matches: &'a ArgMatches) -> Result<Self, Box<dyn Error>>
        where
            Self: Sized,
    {
        Ok(SubCommand { name, matches })
    }


    pub fn name(&self) -> &'a str {
        self.name
    }

    pub fn try_get_default_value_matches(
        &self,
        exclude: Vec<&str>,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        // Scan the matches looking for any that are from the 'default' source.
        // Our goal is to have a hierarchical determination of configuration settings
        // in priority order (highest to lowest):
        // command line argument, environment variable, hctoml file, or default value.
        // So, we need to determine arguments that have default values and determine whether
        // they need to be set with values from the hctoml file, which has a higher priority than
        // default values.
        let default_ids = self
            .matches
            .ids()
            .cloned()
            .filter(|id| {
                self.matches.value_source(id.as_ref()).unwrap() == ValueSource::DefaultValue
            })
            .filter(|id| !exclude.contains(&id.as_str()))
            .map(|id| id.to_string())
            .collect::<Vec<String>>();

        Ok(default_ids)
    }

    pub fn try_get_one_arg<T: Any + Clone + Send + Sync + 'static>(
        &self,
        id: &str,
    ) -> Result<Option<T>, Box<dyn Error>> {
        Ok(self.matches.try_get_one::<T>(id)?.cloned())
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
