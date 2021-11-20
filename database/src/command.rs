/// Database commands from external sources.
/// These are commands that are relative to an `OPEN` database.
pub enum OpenDatabaseCommand {
     /// A command to add a new entry to the database.
     /// Appends the entry to the database if it does not already exist
     /// and returns its Virtual Location.
     New(String, Vec<u8>),
     /// A command to get an item from the database.
     /// Returns the item if it exists, or None if it does not.
     Get(String),
     /// A command to remove an item from the database.
     /// Returns the item if it exists, or None if it does not.
     Remove(String),
     /// A command to update an item in the database.
     /// If it does not exist, it is created.
     /// However if you want to only create it, use `New`.
     Update(String, Vec<u8>)
}

pub enum DatabaseCommand {
    /// A command to open a database.
    /// Where the encapuslated string is the path to the database.
    Open(String),
    /// A command to close a database.
}