#![deny(warnings)]
#![allow(dead_code)]

use std::path::PathBuf;

use derive_builder::Builder;
use serde_derive::Deserialize;

#[derive(Builder, Clone, Debug, Default, Deserialize)]
#[builder(setter(into))]
pub struct Config {
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

#[derive(Builder, Clone, Debug, Default, Deserialize)]
#[builder(setter(into))]
pub struct Directory {
    pub path: Option<PathBuf>,
}

#[derive(Builder, Clone, Debug, Default, Deserialize)]
#[builder(setter(into))]
pub struct Hadoop {
    pub path: Option<PathBuf>,
}
