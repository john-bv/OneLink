# OneLink FS RFC

Updated: **October, 17th 2024**

Author: **John Bergman**



# OVFS (One Link Virtual File System)

This file specifies the One Link Virtual File System and best practices surrounding it.

## 1. Introduction & Motives
The purpose of this document is to provide a standard FS with version control and Atomic Snapshots, without much overhead for the version control implementation. When building OneLink I realized we would need a versioned file system, one thought that instantly came to mind was `.git`, however git lacks a few things, for starters, it's hard to use. `.git` doesn't offer any benefits when it comes to live versioning, each version on git (or commits) needs to be created manually, OVFS fixes this. Another significant advantage over `.git` is OVFS's extended concept of branching and merging from `.git`. You can "merge" two Snapshots together, or branch them off without the negative impact of storage that `.git` would provide.

## 2. Paradigm
OVFS is designed to be a lightweight, efficient, and flexible virtual file system that natively supports automatic, continuous versioning and atomic snapshots. The key paradigm shifts from traditional file systems and version control systems like .git are:

- [Continuous Versioning]()
- [Snapshots]()
  - [Atomic Snapshots]()
  - [System Snapshot]()
- [Buckets]()
- [Chunks](#chunks)
- [Timelines](#timelines)
  - [Versions](#timeline-versions)
  - [Conflicts](#timeline-conflicts)
- [File System]()
  - Blocks
  - Stores
    - Collection
    - Reference
    - Index
  - Sectors
  - Clusters
  - Blobs

## 4. File System

The `FileSystem` is similar to how .git stores files, with a few key changes. The File System has a few layers, on a typical operation, fetch read and write it would look like the following:

> [!CAUTION]
> A `Store` can contain a array of collections as a BTREE. 
> For instance a path like: `/root/users/default/desktop` might have a `ReferenceTable` look up 3 times because it is nested in a directory 3 times, and the last reference table would contain a reference to a `Collection`.

#### Reading `/my/path/my_file.dat`

1. Read Store and check dir `/` -> Read `ReferenceTable` for `/my`.

> [!WARNING]
>
> Every directory contains it's own `Store`, `Timeline`.

2. Read Store at `/` -> `/my` (recurse steps 1 and 2 until in `/path` directory)

3. Resolve reference hash to either a `Timeline` or `Collection`

4. Search `Timeline` for `my_file.dat` and select the time stamp hash relevant to the file you want to read. Typically you want to read the most recent one (so the first one).

5. Index `Collection` and read `my_file.dat` hash from timestamp table.

6. Read `Blob` and dump.

#### Writing a file at `/my/path/test.txt`

> [!CAUTION]
> This example is not recommended for practical use and solely internally. It shows implementation without interacting with `vfs::FileSystem::write(file_path, &blob)` which is what you **SHOULD** use.

```rust
use onelink::vfs::{Blob, Collection, Timeline, Date};

let file_data = Blob::new(&[0]);

// assume timeline is selected from File System
let mut timeline = onelink::vfs::FileSystem::read_dir("/my/path").read_timeline(vfs::CURRENT_TIMELINE);
let mut collection = Collection::new();
let (t_hash, d_hash) = collection.write(&hash, &file);
timeline.write("test.txt", &t_hash);

println!("File created on timeline: {} with hash {} and data location {}", timeline.get_name(), t_hash, d_hash);
```



### Terminology:

- **Blocks:** Blocks represent a way for the OVFS to index on files stored within the VFS. Blocks are `1kb` large.
  Blocks can be indexed by looking at the sectors table.
- **Blobs:** A binary representation of a file. It is used as the "Data" for a file 

## Timelines

Because each directory has it's own **delta**, and uses the same parent timeline, it is easy to cause conflicts between timelines. To avoid conflicts, you can create a `Warp` or "branch" in the `Timeline` that has it's own individual `Collection`'s

> [!NOTE]
> The key difference between a `Warp` and a `Timeline`, differs from the fact a `Timeline` is like a github branch, where a `Warp` is like a branch within the timeline.

A warp, as described above is a **child** of a timeline, and must be manually created. A warp is like a single store for all changes within every nested directory after it. So if I have `/users/john/` as a Warp, `/users/john/desktop` and `/users/john/documents` would now be grouped independently on a timeline that differs from `/users`, meaning `/users/john`, and `/users/` share differing timelines. While this isn't generally an issue, it can cause issues when trying to merge two timelines, because if one timeline has a `Warp` with different deltas from one another, the merge will fail, meaning you will have to chose which `Warp` you want to disassemble and merge into the parent timeline.

> [!WARNING]
> By default, large directories (ones with over `250` entries) will be treated as warps



### Merging Timelines

If you want a directory and it's children to maintain the same "timelines" you can merge a parent and child's timelines together. Think of this like merging `folder/xbox` and `folder/wii` into a new individual folder `folder/games`. Doing this would enable `games` allow `wii` and `xbox` to share the same parent. This is effectively what happens when you merge timelines, except instead of it being the same parent, it is now a single timeline where all changes are recording, while maintaining the same directory. 

> [!CAUTION]
> Merging timelines is **IRREVERSIBLE** in the sense that the original integrity of the timeline can not reasonably be restored.