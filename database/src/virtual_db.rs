use crate::{
    db::{Header, Key},
    preamble::Preamble,
    utils::{GetByteLength, InternalApi},
    DatabaseError,
};
use byteorder::{ReadBytesExt, WriteBytesExt, BE};
use std::{
    fs::File,
    io::{BufReader, Cursor, Read, Seek, Write},
    path::{Path, PathBuf},
};

/// A virtual Item is the "Value" to a key in a One-Link Database.
/// Virtual Items are dropped when used. So if you want to keep the data, you should clone it.
#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

/// This represents a real part.
/// A part is a collection of data in different files.
/// Each part has a magic header and a preamble.
pub struct Partition {
    /// The partition ID.
    /// The partition id is not retrievable from the contents within
    /// the partition itself, but rather the name of the parition file.
    pub id: u8,
    // The data of the partition (in memory).
    // pub data: BufReader<File>,
    /// The length of the data within the partition (in bytes).
    /// This is used by the virtual database to determine the offset of the partition.
    pub length: usize,
    /// Whether or not the partition has been initialized.
    pub initialized: bool,
    /// The path to the partition.
    path: PathBuf,
    /// File handle to the partition.
    file: File,
    /// The start of the partition.
    start: usize,
}

impl Partition {
    pub fn new(base_path: &Path, name: String, id: u8) -> Self {
        let file = File::open(base_path.with_file_name(format!("{}-{}.bin", name, id))).unwrap();
        Self {
            id: id,
            length: 0,
            initialized: false,
            path: base_path.with_file_name(format!("{}-{}.bin", name, id)),
            file,
            start: 0,
        }
    }

    /// Initializes a Partition
    /// This will read the preamble and the magic header and store the starting data offset in memory.
    pub fn init(&mut self) -> Result<(), DatabaseError> {
        let mut buffer = BufReader::new(self.file.try_clone()?);
        let preamble = Preamble::create(&mut buffer.buffer())?;

        buffer.seek_relative(preamble.byte_len() as i64)?;

        if preamble.encryption != 0 {
            // The database is NOT encrypted.
            let header = Header::create(&mut buffer.buffer())?;
            // no need to check virtualization here, we know the header is virtual.
            // we store this start offset incase the db is closed later.
            // however we are going to consume these bytes so they don't take up space.
            self.start = header.byte_len() + preamble.byte_len();
            buffer.take(self.start as u64);
        } else {
            return Err(DatabaseError::Implementation(
                "Non-encrypted databases are not supported yet".to_string(),
            ));
        }
        self.initialized = true;
        Ok(())
    }

    pub fn get_path(&self) -> String {
        self.path.to_str().unwrap_or("").to_string()
    }
}

impl InternalApi for Partition {
    type KeyKind = VirtualKey;
    type ValueKind = VirtualItem;

    fn get(&mut self, key_name: String) -> Result<Self::ValueKind, DatabaseError> {
        // get the keys from the partition
        // we're assuming that the virtual database hasn't cached the address of this key.
        // we're also assuming that the virtual database hasn't cached the data of this key.
        let keys = self.fetch_keys()?;
        let key = keys
            .iter()
            .find(|key| key.name == key_name)
            .ok_or(DatabaseError::KeyNotFound(key_name))?;
        let location = key.location;

        drop(keys);
        // we have the location now we need to get it's data
        // we're going to rely on offset for this.
        let mut buffer = BufReader::new(self.file.try_clone()?);
        buffer.seek_relative(location.offset as i64)?;
        let size = buffer.read_u128::<BE>()?;
        let mut data: Vec<u8> = Vec::with_capacity(size as usize);
        buffer.read_exact(&mut data)?;
        Ok(VirtualItem {
            key: key_name,
            location,
            length: size as usize,
            data,
        })
    }

    fn set(&mut self, key_name: String, value: Vec<u8>) -> Result<Self::KeyKind, DatabaseError> {}

    fn add(&mut self, key_name: String, value: Vec<u8>) -> Result<Self::KeyKind, DatabaseError> {}

    fn remove(&mut self, key_name: String) -> Result<bool, DatabaseError> {}

    /// This operation will reset the handle held on the partition.
    fn fetch_keys(&mut self) -> Result<Vec<Self::KeyKind>, DatabaseError> {}
}

/// The virtual database.
/// We're calling this a virtual database, because everything here is handled virtually. (IN HEAP)
/// Meaning, we need to be careful about what we load and unload into memory.
/// The virtual database will drop everything that is not in use.
pub struct VirtualDatabase {
    /// The parts to the virtual database.
    pub parts: Vec<Partition>,
    /// The keys to the virtual database.
    pub keys: Vec<VirtualKey>,
}

impl VirtualDatabase {
    /// Create a new virtual database.
    pub fn new(header: Header, name: String, path: &Path) -> Self {
        let mut partitions: Vec<Partition> = Vec::new();
        let mut keys: Vec<VirtualKey> = Vec::new();

        // Load the partitions if any.
        if header.partitioned {
            for i in 0..header.partitions.unwrap() {
                partitions.push(Partition::new(path, name.clone(), i as u8));
            }
        }

        Self {
            parts: partitions,
            keys: keys,
        }
    }
}
