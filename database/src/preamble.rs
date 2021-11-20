use crate::DatabaseError;
use crate::MAGIC_BYTES;
use byteorder::{ReadBytesExt, WriteBytesExt, BE};
use std::io::{Cursor, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CompressionMode {
    None,
    Zstd,
}

#[derive(Clone, Debug)]
pub struct Preamble {
    /// The One-Link Database version.
    /// Version 1.0.0 = 100
    pub version: u16,
    /// The compression algorithm used to compress the database.
    pub compression: CompressionMode,
    /// Whether or not the One-Link Database is encrypted.
    /// This is a `u8` to allow extended support for reading encrypted databases.
    /// However currently, this is treated as a boolean.
    pub encryption: u8,
    // /// The checksum of the database.
    // checksum: u32,
}

impl Preamble {
    /// Checks whether the database is encrypted.
    pub fn is_encrypted(self) -> bool {
        self.encryption != 0
    }

    /// Validates that the database is a valid One-Link Database.
    /// This is not reliable for data validation, and is used for header validation.
    pub fn validate_magic(data: &[u8]) -> bool {
        data[0..16] == MAGIC_BYTES
    }

    /// Validates whether the current version of the library supports
    /// the inputed version.
    /// It is important to note that the version is stored as a `u16`
    /// where the hundreds represents the major and and the ten's represent
    /// the minor version.
    pub fn validate_version(version: u16) -> bool {
        match version {
            100 => true,
            _ => false,
        }
    }

    /// Creates a new preamble with the default values.
    /// This assumes that you will encrypt the database.
    /// If you do not encrypt the database, you should use `Preamble::new_unsafe`.
    pub fn new() -> Preamble {
        Preamble {
            version: 100,
            compression: CompressionMode::Zstd,
            encryption: 1,
        }
    }

    /// Creates a new preamble with the default values.
    /// This assumes that you will **not** encrypt the database.
    /// > **STOP:** This is unsafe, and should only be used if you know what you are doing.
    pub fn new_unsafe() -> Preamble {
        Preamble {
            version: 100,
            compression: CompressionMode::Zstd,
            encryption: 0,
        }
    }

    /// Creates a new preamble from the given data.
    /// This is used when reading a database or to validate a database
    /// before writing it.
    pub fn create(data: &[u8]) -> Result<Self, DatabaseError> {
        if data.len() != MAGIC_BYTES.len() + 4 {
            return Err(DatabaseError::PreambleInvalid("Preamble is too short"));
        }

        if !Self::validate_magic(&data[0..16]) {
            return Err(DatabaseError::InvalidDatabase);
        }

        let mut cursor = Cursor::new(data);
        cursor.set_position(16);

        let version = cursor.read_u16::<BE>()?;
        let compression = match cursor.read_u8()? {
            0 => CompressionMode::None,
            1 => CompressionMode::Zstd,
            _ => return Err(DatabaseError::PreambleCompressionInvalid),
        };
        let encryption = cursor.read_u8()?;

        if !Self::validate_version(version) {
            return Err(DatabaseError::InvalidVersion(version));
        } else {
            return Ok(Self {
                version,
                compression,
                encryption,
            });
        }
    }

    /// Writes the preamble to the given writer.
    pub fn write(&self, writer: &mut dyn Write) -> Result<(), DatabaseError> {
        writer.write_all(&MAGIC_BYTES)?;
        writer.write_u16::<BE>(self.version)?;
        writer.write_u8(self.compression as u8)?;
        writer.write_u8(self.encryption)?;
        Ok(())
    }

    /// Gets the byte length of the preamble.
    pub fn byte_len(&self) -> usize {
        MAGIC_BYTES.len() + 4
    }
}
