#![deny(warnings)]
#![allow(dead_code)]

use strum::{Display, EnumString};

#[derive(Clone, Debug, Display, EnumString, PartialEq, Copy)]
pub enum Kind {
    /// Represents a container command in the cli application.
    #[strum(serialize = "container")]
    Container,
    /// Represents a directory command in the cli application.
    #[strum(serialize = "directory")]
    Directory,
    /// Represents a hadoop command in the cli application.
    #[strum(serialize = "hadoop")]
    Hadoop,
}
