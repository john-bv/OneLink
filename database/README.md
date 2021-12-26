# One Link Database
A key-value based database formatted by offsets allowing partial reads once the key's header is consumed.

> Note: This serves a driver for One-Link databases. (For queries, and all)

# Database Format

A One-Link database contains the following parts:

- [One Link Magic](#magic)
- [The Preamble](#1-preamble)
- [Provisional Headers](#2-header)
- [Virtualization](#3-virtualization)



## 1. Preamble

The preamble provides validation to verify that the database file is actually a One-Link database. The preamble reads the magic, as well as some additional data to open the database successfully.

---

### Precedence

The preamble is the **first** set of bytes in a One-Link database.

---

### Size

| Total Bytes | Description                                                  |
| ----------- | ------------------------------------------------------------ |
| 17          | As of One-Link Database v1.0.0 the preamble is 17 bytes long. |

---

### Preamble Binary Structure

| Name         | Type  | Byte Length | Description                                                  |
| ------------ | ----- | ----------- | ------------------------------------------------------------ |
| version      | `u16` | 2           | The Sem-ver version of the database. Currently this is `100` (v1.0.0) |
| *compression | `u8`  | 1           | The compression kind on the database.                        |
| **encryption | `u8`  | 1           | The encryption kind for the database. Currently this is treated as a `bool`. |

> ##### Key
>
> | Symbol / Name | Description                                                  |
> | ------------- | ------------------------------------------------------------ |
> | *             | Byte is represented as [CompressionVariant]().               |
> | **            | Byte is represented as [EncryptionVariant](#device-operating-system). |
>
> #### Struct representation
>
> ```rust
> pub struct Preamble {
>     version: u16,
>     compression: u8,
>     encryption_level: u8
> }
> ```

### Magic

The magic is a series of bytes which contain a `UTF-8 String` that matches `OneLink VERSION` where `VERSION` is `1.0.0`.

Magic is the first series of bytes used in the preamble to validate that the One-Link Database is correct.

```rust
pub const MAGIC_BYTES: [u8; 13] = [79, 110, 101, 108, 105, 110, 107, 32, 49, 46, 48, 46, 48];
```

## 



## 2. Header

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
> #### Struct Representation
>
> ```rust
> pub struct Header {
>     pub partitioned: bool,
>     pub partition_index: Option<u8>,
>     pub partitions: Option<u8>,
>     
>     /// -- META DATA --
>     pub created_on: u8,
>     pub last_open: u128,
>     pub last_close: u128,
>     pub last_write: u128,
>     pub virtualization: bool,
> }
> ```
