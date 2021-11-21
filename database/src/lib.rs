pub mod db;
pub mod preamble;
pub mod single_db;
pub mod utils;
pub mod virtual_db;

/// An array of "Magic" bytes, which represents this
/// Set is a valid database for onelink.
/// MAGIC is a set of bytes that match "OneLink 1.0.0"
/// "OneLink 1.0.0" as bytes in decimal:
pub const MAGIC_BYTES: [u8; 13] = [79, 110, 101, 108, 105, 110, 107, 32, 49, 46, 48, 46, 48];

/// A function to generate an array of "Magic" bytes in relevance to One-Link.
/// This can be used to validate `MAGIC_BYTES`.
pub fn generate_magic() -> Vec<u8> {
    "OneLink 1.0.0".as_bytes().to_vec()
}

#[derive(Debug)]
#[repr(u8)]
pub enum DatabaseError {
    /// The database is encrypted, and the password is incorrect.
    IncorrectPassword,

    /// The database preamble is invalid. This error is generic, and
    /// encapsulates the specific error.
    PreambleInvalid(&'static str),

    /// The preamble contains an invalid compression mode.
    /// Currently only `0` and `1` are supported.
    /// Where `0` is `CompressionMode::None` and `1` is `CompressionMode::Zstd`
    PreambleCompressionInvalid,

    /// The database is not a valid One-Link Database.
    /// This happens when the magic bytes are incorrect.
    InvalidDatabase,

    /// The database version is not supported by this version of the library.
    /// The version provided is encapsulated.
    InvalidVersion(u16),

    /// Another issue occurred that was related to an IO operation.
    /// The specific error is encapsulated.
    IoError(std::io::Error),

    /// An issue related to implementation of the library.
    /// This error is encapsulated.
    Implementation(String),

    /// The following is related to the database read/write operations.
    /// When the database key is not found. The encapsulated key is the key that was not found.
    KeyNotFound(String),
}

impl From<std::io::Error> for DatabaseError {
    fn from(error: std::io::Error) -> Self {
        DatabaseError::IoError(error)
    }
}
