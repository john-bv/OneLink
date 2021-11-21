# One Link Database
A key-value based database formatted by offsets allowing partial reads once the key's header is consumed.

> Note: This serves a driver for One-Link databases. (For queries, and all)

# Database Format

A One-Link database contains the following parts:

- [One Link Magic](#magic)
- [The Preamble](#preamble)
- [Provisional Headers](#header)
- [Virtualization](#virtualization)

## Magic

The magic is a series of bytes which contain a `UTF-8 String` that matches `OneLink VERSION` where `VERSION` is `1.0.0`.

```rust
pub const MAGIC_BYTES: [u8; 13] = [79, 110, 101, 108, 105, 110, 107, 32, 49, 46, 48, 46, 48];
```

## Preamble

The **preamble** of a OneLink database is **20** bytes in length, and contains the necessary information for validating the contents of the database, as well as decrypting the contents (if encryption is set). The preamble includes the Magic bytes required for validating a OneLink database.

```rust
pub struct Preamble {
    /// A 2 byte integer, representing the Major to Minor version
    /// Where `1.0.0` would be `100` and `1.3.9` would be `139`.
    version: u16,
    /// A single byte where:
    /// 0 - No Compression
    /// 1 - Zstd Compression
    /// 2 - Zlib Compression
    /// 3 - OneLinkDelta Compression
    compression: u8,
    /// A single byte where:
    /// 0 - No Encrpytion
    /// 1 - Sha256 rc3
    /// 2 - AES
    /// 3 - Two Fish
    encryption_level: u8
}
```

## Header

### Information

A OneLink header contains the information required to **read** the contents of the database. The header will also exist on partitions and should be read the same as the host database.

---

### Precedence 

The header is retrievable once the [database has been decrypted]().

---

### Size

| Total Bytes | Description                                      |
| ----------- | ------------------------------------------------ |
| 24          | The header is 24 bytes if it is not partitioned. |
| 26          | The header is 26 bytes if it is partitioned.     |

---

### Header Binary Structure

| Name             | Type   | Byte Length | Description                                                  |
| ---------------- | ------ | ----------- | ------------------------------------------------------------ |
| partitioned      | `bool` | 1           | Whether or not the database is partitioned.                  |
| *partition_index | `u8`   | 1           | The partition index in the vector of partitions.             |
| *partition_size  | `u8`   | 1           | The total size of partitions to be expected.                 |
| **created_on     | `u8`   | 1           | The operating system the database was created on.            |
| last_opened      | `u128` | 8           | The unix epoch time stamp that the database was last opened. |
| last_close       | `u128` | 8           | The unix epoch time stamp that the database was last closed. |
| last_write       | `u128` | 8           | The unix epoch time stamp that the database was last modified. |
| virtualized      | `bool` | 1           | Whether or not the database is virtualized.                  |

> ##### Key
>
> | Symbol / Name | Description                                                  |
> | ------------- | ------------------------------------------------------------ |
> | *             | If `partitioned` is true.                                    |
> | **            | Byte is represented as [DeviceOperatingSystem](#device-operating-system). |
>
> 

```rust
pub struct Header {
    pub partitioned: bool,
    pub partition_index: Option<u8>,
    pub partitions: Option<u8>,
    
    /// -- META DATA --
    pub created_on: u8,
    pub last_open: u128,
    pub last_close: u128,
    pub last_write: u128,
    pub virtualization: bool,
}
```



