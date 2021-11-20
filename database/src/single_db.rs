use crate::db::Key;

// All data is raw, and will be loaded into memory with a single database.
// For a more scalar approach, use a `VirtualDatabase`.

pub struct SingleDatabase {
    pub keys: Vec<Key>,
}
