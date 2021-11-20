use byteorder::{ReadBytesExt, BE};
use std::any::Any;
use std::fs::File;
use std::io::BufReader;
use std::io::Cursor;

use crate::preamble::Preamble;
use crate::virtual_db::VirtualDatabase;
use crate::DatabaseError;

pub enum DbDeviceOs {
    Linux,
    Windows,
    Mac,
    Unknown,
}

pub trait InternalApi {
    /// The key type used by the internal adapter for this database.
    type KeyKind;

    /// Get a key from the database.
    fn get(&self, key_name: String) -> Result<Self::KeyKind, DatabaseError>;

    /// Set a key in the database.
    /// This will overwrite any existing key.
    /// If you want to add a new key, use `add_key`.
    fn set(&mut self, key_name: String, value: Vec<u8>) -> Result<Self::KeyKind, DatabaseError>;

    /// Similar to `set`, but will only create the key if it does not already exist.
    fn add(&mut self, key_name: String, value: Vec<u8>) -> Result<Self::KeyKind, DatabaseError>;

    /// Remove a key from the database.
    /// Returns whether or not the operation succeeded.
    fn remove(&mut self, key_name: String) -> Result<bool, DatabaseError>;

    /// Fetch all keys from the database.
    /// Returns a vector of keys.
    fn fetch_keys() -> Result<Vec<Self::KeyKind>, DatabaseError>;
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
pub struct Header {
    /// Whether or not the virtual database is partitioned
    /// Partitions should only be set if you wish to split the database into multiple files.
    pub partitioned: bool,
    /// If partitioned, the partition index for this partition
    pub partition_index: Option<u32>,
    /// If partitioned, the number of partitions
    pub partitions: Option<u32>,
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
            Some(cursor.read_u32::<BE>()?)
        } else {
            None
        };
        let partitions = if partitioned {
            Some(cursor.read_u32::<BE>()?)
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
    pub fn open(relative_path: String) -> Result<Database, DatabaseError> {
        let mut db_file = File::open(&relative_path)?;
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
                    "Non-Virtualized database are not supported".to_string(),
                ));
            }
            Ok(Database {
                preamble,
                header,
                mode,
                internal: InternalDatabase::Virtual(VirtualDatabase::new(
                    relative_path,
                    header.partitioned,
                    header.partition_index,
                    header.partitions,
                )?),
            })
        }
    }
}
