# Onelink VFS

OneLink's local Virtual File System.

# Documentation

1. Creating a snapshot



## Creating Snapshots

The fundamentals behind a snapshot remain the same concept as a `.git` commit. It's a "snapshot" of the current path at that current point in time.

This is useful for merging conflicting files when synchronizing updates from the server, which will happen from time to time.

```rust
use onelink_vfs::FileSystem;
use onelink_vfs::RefTypes;

fn main() {
    let mut fs = FileSystem::new("/local/path");
    fs.mount("example", "https://onelink.dev/shares/onelink/EXAMPLE_COLLECTION.ovfs").unwrap();
    
    // We are now mounted to the fs
    // Before we can do anything, we need to select a disk
    let mut online_disk: FileSystem = fs.mounts("example").unwrap();
    
    // now that we have a disk, we can read/write to it (given we're allowed to)
    // because of this, we can make a snapshot of everything on the drive, and export it locally.
    let mut snapshot_opts = SnapShotBuilder::new()
    	.history(date!("30d"))
    	.refs(RefsTypes::RESOLVE)
    	.origin(true); // setting this to false will make this merely a backup.
    let mut snapshot = online_disk.snapshot("/");
    // we've now mounted this snapshot
    let mut backup_disk = fs.mount("backup", snapshot.into());
    
    // we can do more though...
    // we can change the state of the snapshot, depending on how many records are stored within.
    // we can get a specific record with snapshot.get_records(), records are like git commits.
    snapshot.set_record("4468e5deabf5e6d0740cd1a77df56f67093ec943");
    
    for f in backup.walk('/') {
        println!("{}: with ref: {}", f.path, f.record());
    }
    
    // let's write to this backup
    // the mounted disk is updated in real time.
    backup.write("/path/to/file.txt", text!("hello world!"), Permissions::ALL);
    
    // We now have diverging paths, we need to make a snapshot to converge paths
    let snapshot = backup_disk.snapshot("/");
    // we need to inject this snapshot 
}
```



### Creating a new Snapshot



```rust
use onelink_vfs::FileSystem;

async fn mount() {
    let mut fs = FileSystem::new("Name");
    fs.mount("./");
    
    // Onelink filesystem contains more features than your
    // normal file system.
    
    // create a snapshot of a directory
    let snapshot = fs.snapshot("/path/to/dir");
    // or a file
    let snapshot = fs.snapshot("/path/to/file", SnapShot::FILE);

    // Here's a powerful usecase of a snapshot
    // let's get a file, we'll call it "hello world"
    let mut file = fs.create_file("hello world.txt", &[]);
    // make the file record changes with snapshots
    file.set_snapshots(true);
    
    // let's modify the file
    file.write(&b"hello world!");
    
    // let's write to it again.
    file.write(&b"goodbye world!");
    
    // our file now only contains "goodbye world!".
    // but because snapshots are enabled, we can retrieve the last change!
    let snapshots = file.snapshots().await; // or file.snapshots();
    
    // Writes "hello world!"
    file.write(&snapshots.recent().blob());

    // remove the snapshot from records
    snapshots.remove(snapshots.recent().hash());
}
```