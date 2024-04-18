#![deny(warnings)]
#![allow(dead_code)]

use clap::parser::ValueSource;
use clap::ArgMatches;
use phf::phf_map;
use std::any::Any;
use std::error::Error;

#[derive(Clone, Debug, PartialEq)]
pub enum SubCommandKind {
    /// Represents a container command in the cli application.
    Container,
    /// Represents a directory command in the cli application.
    Directory,
    /// Represents a hadoop command in the cli application.
    Hadoop,
}

static SUBCOMMAND_KIND: phf::Map<&'static str, SubCommandKind> = phf_map! {
    "container"      => SubCommandKind::Container,
    "directory"      => SubCommandKind::Directory,
    "hadoop"         => SubCommandKind::Hadoop,
};

pub fn parse_kind(kind: &str) -> Option<SubCommandKind> {
    SUBCOMMAND_KIND.get(kind).cloned()
}

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

// &str to SubCommandEnum
// SubCommandEnum to &str

impl<'a> SubCommand<'a> {
    pub fn name(&self) -> &'a str {
        self.name
    }

    // pub fn try_from<String> for Arg<bool> {
    //     type Error = Box<dyn std::error::Error>;
    //
    //     fn try_from(original_value: String) -> Result<Self, Self::Error> {
    //         let v: Arg<bool> = Arg {
    //             //name: "".to_string(),
    //             original_value: original_value.to_string(),
    //             original_kind: ArgKind::String,
    //             converted_kind: ArgKind::Bool,
    //             converted_value: original_value.parse::<bool>()?,
    //         };
    //
    //         Ok(v)
    //     }
    // }
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
        Ok(self.matches.try_get_one::<T>(id)?.cloned())
    }
}
