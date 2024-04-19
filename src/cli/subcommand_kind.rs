#![deny(warnings)]
#![allow(dead_code)]

use clap::parser::ValueSource;
use clap::ArgMatches;
use phf::phf_map;
use std::any::Any;
use std::error::Error;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum SubCommandKind {
    /// Represents a container command in the cli application.
    Container,
    /// Represents a directory command in the cli application.
    Directory,
    /// Represents a hadoop command in the cli application.
    Hadoop,
}

pub(crate) static SUBCOMMAND_KIND: phf::Map<&'static str, SubCommandKind> = phf_map! {
    "container"      => SubCommandKind::Container,
    "directory"      => SubCommandKind::Directory,
    "hadoop"         => SubCommandKind::Hadoop,
};

pub(crate) fn parse_kind(kind: &str) -> Option<SubCommandKind> {
    SUBCOMMAND_KIND.get(kind).cloned()
}
