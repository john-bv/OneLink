use crate::DatabaseError;

/// A utility trait to get the amount of bytes of a certain database struct.
pub trait GetByteLength {
    /// Gets the length of the struct in bytes.
    fn byte_len(&self) -> usize;
}

pub trait InternalApi {
    /// The key type used by the internal adapter for this database.
    type KeyKind;
    /// The value type used by the internal adapter for this database.
    type ValueKind;

    /// Get a key from the database.
    fn get(&mut self, key_name: String) -> Result<Self::ValueKind, DatabaseError>;

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
    fn fetch_keys(&mut self) -> Result<Vec<Self::KeyKind>, DatabaseError>;
}
