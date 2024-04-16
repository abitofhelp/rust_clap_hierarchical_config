#![deny(warnings)]
#![allow(dead_code)]

use clap::parser::ValueSource;
use clap::ArgMatches;
use std::any::Any;
use std::error::Error;

pub trait SubCommandTrait<'a> {
    fn new(
        name: &'a str,
        matches: &'a ArgMatches,
    ) -> anyhow::Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;

    fn get_default_value_matches(
        &self,
        exclude: Vec<&str>,
    ) -> anyhow::Result<Vec<String>, Box<dyn Error>>;

    fn get_one_arg<T: Any + Clone + Send + Sync + 'static>(
        &self,
        arg_id: &str,
    ) -> anyhow::Result<Option<T>, Box<dyn std::error::Error>>;
}

#[derive(Clone, Debug)]
pub(crate) struct SubCommand<'a> {
    name: &'a str,
    matches: &'a ArgMatches,
}
impl<'a> SubCommandTrait<'a> for SubCommand<'a> {
    fn new(name: &'a str, matches: &'a ArgMatches) -> anyhow::Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        Ok(SubCommand { name, matches })
    }
    fn get_default_value_matches(
        &self,
        exclude: Vec<&str>,
    ) -> anyhow::Result<Vec<String>, Box<dyn Error>> {
        // Scan the matches looking for any that are from the 'default' source.
        // Our goal is to have a hierarchical determination of configuration settings
        // in priority order (highest to lowest):
        // command line argument, environment variable, toml file, or default value.
        // So, we need to determine arguments that have default values and determine whether
        // they need to be set with values from the toml file, which has a higher priority than
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

    fn get_one_arg<T: Any + Clone + Send + Sync + 'static>(
        &self,
        id: &str,
    ) -> anyhow::Result<Option<T>, Box<dyn Error>> {
        let v = self.matches.get_one::<T>(id).cloned();
        Ok(v)
    }
}
