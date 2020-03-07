use crate::ProtocolVersion;

/// Represents a provider which converts networking primitives
/// into application-specific types, including chunks, blocks,
/// items, etc.
pub trait Provider: Send + Sync + 'static {
    /// The chunk type for this application.
    type Chunk: Clone + Send + Sync;
    /// The chunk section type for this application.
    type ChunkSection: Clone + Send + Sync;
    /// The block type for this application.
    type Block: Clone + Send + Sync;
    /// The item type for this application.
    type Item: Clone + Send + Sync;
    /// The error type.
    type Error: std::error::Error;

    /// Converts a chunk into an array of 10 chunk sections.
    fn chunk_sections(chunk: Self::Chunk) -> [Option<Self::ChunkSection>; 10];
    /// Returns the position of a chunk.
    fn chunk_position(chunk: &Self::Chunk) -> (i32, i32);
    /// Serializes a chunk section into a `RawChunkSection`.
    fn serialize_chunk_section(section: &Self::ChunkSection) -> RawChunkSection;
    /// Deseriazes a `RawChunkSection` into a chunk section.
    fn deserialize_chunk_section(raw: RawChunkSection) -> Result<Self::ChunkSection, Self::Error>;

    /// Converts a block ID from the section or global palette
    /// into the block ID for the given protocol version.
    fn convert_block_state_id(version: ProtocolVersion, id: u16) -> u16;

    /// Serializes a block into a raw block ID.
    fn serialize_block(block: Self::Block) -> u16;
    /// Deserializes a raw block ID into a block.
    fn deserialize_block(id: u16) -> Result<Self::Block, Self::Error>;

    /// Serializes an item into a raw item ID.
    fn serialize_item(item: Self::Item) -> u16;
    /// Deserializes a raw item ID into an item.
    fn deserialize_item(id: u16) -> Result<Self::Item, Self::Error>;
}

/// A generic, serialized form of a raw chunk section.
pub struct RawChunkSection<'a> {
    /// The number of bits per block used by this chunk.
    pub bits_per_block: u8,
    /// The number of non-air blocks in this chunk, i.e. the number
    /// of blocks which are neither air, cave air, or void air.
    pub non_air_blocks: u16,
    /// The palette for this chunk.
    pub palette: RawChunkPalette<'a>,
    /// The data array, which consists of a compacted
    /// list of 4096 longs pointing to entries in the palette.
    pub data: &'a [u64],
}

/// Palette for a raw chunk.
pub enum RawChunkPalette<'a> {
    /// Global palette.
    Global,
    /// Section paleete.
    Section(&'a [u16]),
}