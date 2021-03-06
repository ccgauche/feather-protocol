mod network;
mod error;
mod message;
mod packets;
mod provider;
pub mod types;

pub use error::Error;
pub use message::Message;
pub use provider::{Provider, RawChunkPalette, RawChunkSection};

/// Protocol version.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProtocolVersion {
    V1_13_2,
    V1_14_4,
    V1_15_0,
    V1_15_1,
    V1_15_2,
}

pub mod v1_14_4 {
    // feather_protocol_codegen::feather_protocol!("1.14.4");
}

pub mod v1_15_0 {
    // feather_protocol_codegen::feather_protocol!("1.15");
}

pub mod v1_15_1 {
    // feather_protocol_codegen::feather_protocol!("1.15.1");
}

pub mod v1_15_2 {
    // feather_protocol_codegen::feather_protocol!("1.15.2");
}
