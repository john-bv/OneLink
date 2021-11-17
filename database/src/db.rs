use crate::preamble::Preamble;
use crate::virtual_db::VirtualDatabase;
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
     Virtual
}

/// The database header.
/// This is instiantiated after the database is opened.
pub struct Header {
     /// Whether or not the virtual database is partitioned
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
     /// Partitions should only be set if you wish to split the database into multiple files.
     pub virtualization: bool
}

pub struct Key {
     /// The name of the key.
     pub name: String,
     /// The Index of the value in the database. (This is a virtual index)
     pub index: u64,
     /// The Offset of the value in the database.
     pub offset: u64
}

pub struct Database {
     /// The opening header to the database.
     pub preamble: Preamble,
     /// The header of the database.
     pub header: Header,
     /// The mode of the database.
     pub mode: DatabaseMode,
     /// The virtual database (if virtualized by parts).
     v: Option<VirtualDatabase>,
     /// The contents of the database (if not virtualized).
     contents: Vec<u8>,
     /// The "key" or "password" used to open the database.
     password: Option<String>,
}