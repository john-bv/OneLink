# One Link Database
A key-value based database formatted by offsets allowing partial reads once the key's header is consumed.

> Note: This serves a driver for One-Link databases. (For queries, and all)

# Database Format

A One-Link database contains the following parts:

- [One Link Magic](#magic)
- [The Preamble](#preamble)
- [Provisional Headers](#header)

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

A OneLink header contains the information required to **read** the contents of the database. The header is retrievable once the database has been decrypted.

