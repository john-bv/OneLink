use crate::db::{InternalApi, Key};

/// The virtual database.
/// We're calling this a virtual database, because everything here is handled virtually. (IN HEAP)
/// Meaning, we need to be careful about what we load and unload into memory.
/// The virtual database will drop everything that is not in use.
pub struct VirtualDatabase {
    /// The parts to the virtual database.
    pub parts: Vec<Partition>,
    /// The keys to the virtual database.
    pub keys: Vec<Key>,
}

impl InternalApi for VirtualDatabase {
    type KeyKind = VirtualKey;
}

/// This represents a real part.
/// A part is a collection of data in different files.
pub struct Partition {
    /// The partition ID.
    /// The partition id is not retrievable from the contents within
    /// the partition itself, but rather the name of the parition file.
    pub id: u8,
    /// The data of the partition (in memory).
    pub data: Vec<u8>,
    /// The length of the data within the partition (in bytes).
    /// This is used by the virtual database to determine the offset of the partition.
    pub length: usize,
}

impl Partition {
    pub fn new() -> Self {
        Self {
            id: 0,
            data: Vec::new(),
            length: 0,
        }
    }

    pub fn get_name(&self) -> String {
        format!("part-{}", self.id)
    }
}

/// A virtual Item is the "Value" to a key in a One-Link Database.
pub struct VirtualItem {
    /// The name the item is stored under.
    pub key: String,
    /// The location of the part in the database.
    pub location: VirtualLocation,
    /// The length of the part in bytes.
    pub length: usize,
    /// The data of the item.
    pub data: Vec<u8>,
}

pub struct VirtualKey {
    /// The name of the key.
    pub name: String,
    /// The location of the part in the database.
    pub location: VirtualLocation,
    /// The length of the part in bytes.
    pub length: usize,
}

pub struct VirtualLocation {
    /// The partition index of the virtual location
    pub id: u64,
    /// The offset of the virtual location (relative to the partition)
    pub offset: u64,
    /// The index of the virtual location.
    /// This is slower, but more reliable because it is not based on the offset.
    /// The index is the location of the item in the partition. (by block)
    /// ```rust ignore
    /// // open a database that already exists
    /// let mut db = Database::open("test_virtual.onelink", true).unwrap();
    /// let mut item = db.query(GET "foo").unwrap();
    /// if item.data != "bar".as_bytes() {
    ///    panic!("The data is not what we expected!");
    /// }
    /// ```
    pub index: u64,
}

impl VirtualLocation {
    /// Creates a new virtual Location. (with a default offset of 0 and a default id of 0)
    pub fn new() -> Self {
        VirtualLocation {
            id: 0,
            offset: 0,
            index: 0,
        }
    }
}
