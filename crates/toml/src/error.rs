use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigFileError {
    #[error("the field '{field_name:?}' was not found in the hctoml configuration file table '{table_name:?}'")]
    FieldNotFound {
        table_name: Option<String>,
        field_name: String,
    },

    #[error("the hctoml configuration file '{path:?}' was not found")]
    NotFound { path: String },

    #[error("unknown hctoml configuration file error")]
    Unknown,
}