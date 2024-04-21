use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigFileError {
    #[error("the field '{field_name:?}' was not found in the toml configuration file table '{table_name:?}'")]
    FieldNotFound {
        table_name: Option<String>,
        field_name: String,
    },

    #[error("the toml configuration file '{path:?}' was not found")]
    NotFound { path: String },

    #[error("unknown toml configuration file error")]
    Unknown,
}
