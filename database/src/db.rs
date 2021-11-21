use byteorder::{ReadBytesExt, BE};
use std::any::Any;
use std::fs::File;
use std::io::BufReader;
use std::io::Cursor;
use std::path::Path;

use crate::preamble::Preamble;
use crate::utils::GetByteLength;
use crate::virtual_db::VirtualDatabase;
use crate::DatabaseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DbDeviceOs {
    Linux,
    Windows,
    Mac,
    Unknown,
}

/// The One-Link database mode.
/// This will not effect the api, however it will change
/// the behaviour of the database.
pub enum DatabaseMode {
    Single,
    Virtual,
}

/// The database header.
/// This is instiantiated after the database is opened.
#[derive(Debug, Clone)]
pub struct Header {
    /// Whether or not the virtual database is partitioned
    /// Partitions should only be set if you wish to split the database into multiple files.
    pub partitioned: bool,
    /// If partitioned, the partition index for this partition
    pub partition_index: Option<u8>,
    /// If partitioned, the number of partitions
    pub partitions: Option<u8>,
    // The following is metadata
    /// The Operating System that created the One-Link Database.
    pub created_on: DbDeviceOs,
    /// The time of the last "open" operation.
    pub last_open: u128,
    /// The time of the last "close" operation.
    pub last_close: u128,
    /// The time of the last "write" operation.
    pub last_write: u128,
    /// Whether or not the OneLink Database should be virtualized.
    /// This needs to be `true` for partitions.
    /// However it is recommended for all instances.
    pub virtualization: bool,
}

impl Header {
    /// Creates a header from the given data.
    /// This is used when opening a database.
    pub fn create(data: &[u8]) -> Result<Header, DatabaseError> {
        let mut cursor = Cursor::new(data);
        let partitioned = cursor.read_u8()? != 0;
        let partition_index = if partitioned {
            Some(cursor.read_u8()?)
        } else {
            None
        };
        let partitions = if partitioned {
            Some(cursor.read_u8()?)
        } else {
            None
        };
        let created_on = match cursor.read_u8()? {
            0 => DbDeviceOs::Linux,
            1 => DbDeviceOs::Windows,
            2 => DbDeviceOs::Mac,
            _ => DbDeviceOs::Unknown,
        };
        let last_open = cursor.read_u128::<BE>()?;
        let last_close = cursor.read_u128::<BE>()?;
        let last_write = cursor.read_u128::<BE>()?;
        let virtualization = cursor.read_u8()? != 0;

        Ok(Self {
            partitioned,
            partition_index,
            partitions,
            created_on,
            last_open,
            last_close,
            last_write,
            virtualization,
        })
    }

    pub fn byte_len(&self) -> usize {
        let mut current_byte_size: usize = 1;
        if self.partitioned {
            // account for the partition index and partitions count
            current_byte_size += 2;
        }
        // db os is 1 byte
        // last open is 8 bytes, last close is 8 bytes, last write is 8 bytes
        // virtualization is 1 byte
        current_byte_size += 26;
        return current_byte_size;
    }
}

pub struct Key {
    /// The name of the key.
    pub name: String,
    /// The Index of the value in the database. (This is a virtual index)
    pub index: u64,
    /// The Offset of the value in the database.
    pub offset: u64,
}

pub enum InternalDatabase {
    Single(File),
    Virtual(VirtualDatabase),
}

impl InternalDatabase {
    pub fn inner(&self) -> &dyn Any {
        match self {
            InternalDatabase::Single(file) => file,
            InternalDatabase::Virtual(virtual_db) => virtual_db,
        }
    }
}

pub struct Database {
    /// The opening header to the database.
    pub preamble: Preamble,
    /// The header of the database.
    pub header: Header,
    /// The mode of the database.
    pub mode: DatabaseMode,
    /// The virtual database.
    internal: InternalDatabase,
}

impl Database {
    /// Opens a One-Link Database.
    /// This will open the database and read the headers.
    pub fn open(name: String, path: String) -> Result<Database, DatabaseError> {
        let mut db_file = File::open(&path)?;
        let mut buffer = BufReader::new(db_file);
        let preamble = Preamble::create(&mut buffer.buffer())?;
        let mut mode = DatabaseMode::Single;

        buffer.seek_relative(preamble.byte_len() as i64)?;

        if preamble.encryption != 0 {
            // The database is NOT encrypted.
            let header = Header::create(&mut buffer.buffer())?;

            if header.virtualization {
                // The database is virtualized.
                mode = DatabaseMode::Virtual;
            } else {
                return Err(DatabaseError::Implementation(
                    "Non-Virtualized databases are not supported yet".to_string(),
                ));
            }
            Ok(Database {
                preamble,
                header: header.clone(),
                mode,
                internal: InternalDatabase::Virtual(VirtualDatabase::new(
                    header,
                    name,
                    Path::new(&path),
                )),
            })
        } else {
            Err(DatabaseError::Implementation(
                "Non-encrypted databases are not supported yet".to_string(),
            ))
        }
    }
}
