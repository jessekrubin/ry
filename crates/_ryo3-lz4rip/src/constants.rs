//! lz4 constants

/// LZ4 frame magic number (little-endian on the wire)
pub const LZ4F_MAGIC: [u8; 4] = [0x04, 0x22, 0x4D, 0x18];
/// FLG byte bit signalling that the header stores the uncompressed size
pub const LZ4F_FLG_CONTENT_SIZE: u8 = 0b0000_1000;
