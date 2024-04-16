#![deny(warnings)]
#![allow(dead_code)]

use serde_derive::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub debug: Option<bool>,
    pub container: Container,
    pub directory: Directory,
    pub hadoop: Hadoop,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Container {
    pub name: Option<String>,
}

impl Container {
    fn new() -> Self {
        Container { name: None }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Directory {
    pub path: Option<PathBuf>,
}

impl Directory {
    fn new() -> Self {
        Directory { path: None }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Hadoop {
    pub path: Option<PathBuf>,
}

impl Hadoop {
    fn new() -> Self {
        Hadoop { path: None }
    }
}

impl Config {
    pub fn new() -> Self {
        Config {
            debug: None,
            container: Container::new(),
            directory: Directory::new(),
            hadoop: Hadoop::new(),
        }
    }
}
