#![deny(warnings)]
#![allow(dead_code)]

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SubCommandError {
    #[error("unknown subcommand kind was encountered")]
    Unknown,
}