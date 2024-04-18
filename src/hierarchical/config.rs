#![deny(warnings)]
#![allow(dead_code)]

use serde_derive::Deserialize;
use std::path::PathBuf;
use derive_builder::Builder;

#[derive(Builder, Clone, Debug, Default, Deserialize)]
#[builder(setter(into))]
pub struct Config {
    // FIXME: Need Option<> if not using builder pattern!
    pub debug: Option<bool>,
    pub container: Option<Container>,
    pub directory: Option<Directory>,
    pub hadoop: Option<Hadoop>,
}

#[derive(Builder, Clone, Debug, Default, Deserialize)]
#[builder(setter(into))]
pub struct Container {
    pub name: Option<String>,
}

impl Container {
    fn new() -> Self {
        Container { name: None }
    }
}

#[derive(Builder, Clone, Debug, Default, Deserialize)]
#[builder(setter(into))]
pub struct Directory {
    pub path: Option<PathBuf>,
}

impl Directory {
    fn new() -> Self {
        Directory { path: None }
    }
}

#[derive(Builder, Clone, Debug, Default, Deserialize)]
#[builder(setter(into))]
pub struct Hadoop {
    pub path: Option<PathBuf>,
}

impl Hadoop {
    fn new() -> Self {
        Hadoop { path: None }
    }
}

impl Config {
    // pub fn new() -> Self {
    //     Config {
    //         debug: None,
    //         container: Container::new(),
    //         directory: Directory::new(),
    //         hadoop: Hadoop::new(),
    //     }
    // }
}
