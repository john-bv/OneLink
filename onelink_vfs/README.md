# Onelink VFS

OneLink's local Virtual File System.



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