
pub use thiserror::Error;
use std::io;

#[derive(Debug, Error)]
pub enum Error {
    /// Invalid Magic Number
    #[error("Invalid EOF magic number")]
    InvalidMagic,

    /// Triggers when the EOF version is invalid
    #[error("Invalid EOF version")]
    InvalidVersion,

    /// Triggers when the number of code sections is invalid: a lot of reasons of this
    #[error("Invalid number of code sections")]
    InvalidCodeSectionCount,

    /// Triggers when the section size is invalid
    #[error("Invalid type section size")]
    InvalidTypeSectionSize,

    /// Invalid Metadata
    #[error("Invalid metadata for 0th code section")]
    InvalidZeroSectionMetadata,

    /// Parsing Error
    #[error("Parsing error: {0}")]
    ParseError(String),

    /// I/O Error
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
}
