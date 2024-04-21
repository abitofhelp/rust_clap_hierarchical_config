use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("the cli subcommand was not present at runtime")]
    SubCommandNotPresent,

    #[error("unknown application error")]
    Unknown,
}
