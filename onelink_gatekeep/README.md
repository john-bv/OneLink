# Gatekeeper

The onelink gatekeeper is a special module that provides the event i/o for local changes. It's almost like a relay server for events, but locally. The gatekeeper will keep and stage events as they happen as individual requests until you are connected and ready to synchronize your data.



```rust
use onelink_gatekeep as gatekeep;

async fn stage() {
    let mut keeper = gatekeep::GateKeeper::new(".gatekeep");
    let event = keeper.stage(gatekeep::event::MODIFY);
    // add_blob(File, blob)
    // the first parameter here represents the id within the VFS.
    // these are mapped to ensure absolute directory paths, and better save
    // events. Files are assigned an id only by the server, so if a file is new
    // it can NOT be considered virtual yet, and therefore has a id 
    event.add_blob("FILE_ID", &[0, 12]);
}
```