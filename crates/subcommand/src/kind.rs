#![deny(warnings)]
#![allow(dead_code)]

use phf::phf_map;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Kind {
    /// Represents a container command in the cli application.
    Container,
    /// Represents a directory command in the cli application.
    Directory,
    /// Represents a hadoop command in the cli application.
    Hadoop,
}

static SUBCOMMAND_KIND: phf::Map<&'static str, Kind> = phf_map! {
    "container"      => Kind::Container,
    "directory"      => Kind::Directory,
    "hadoop"         => Kind::Hadoop,
};

pub fn parse_kind(kind: &str) -> Option<Kind> {
    SUBCOMMAND_KIND.get(kind).cloned()
}
