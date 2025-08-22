use crate::conn::protocol::{
    codec::decode::{
        PacketDecode,
        DecodeBuf
    },
    packet::{
        PacketMeta,
        PacketState,
        PacketBound
    }
};


#[derive(Debug)]
pub struct RequestPacket;

impl PacketMeta for RequestPacket {
    const STATE  : PacketState = PacketState::Status;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x00; // TODO: Check against current datagen.
}

impl PacketDecode for RequestPacket {
    type Error = !;

    fn decode(_ : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    { Ok(Self) }
}
