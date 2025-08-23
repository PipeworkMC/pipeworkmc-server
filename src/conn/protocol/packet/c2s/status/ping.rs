use crate::conn::protocol::{
    codec::decode::{
        PacketDecode,
        DecodeBuf,
        IncompleteDecodeError
    },
    packet::{
        PacketMeta,
        PacketState,
        PacketBound
    }
};


#[derive(Debug)]
pub struct C2SStatusPingPacket {
    pub timestamp : u64
}

impl PacketMeta for C2SStatusPingPacket {
    const STATE  : PacketState = PacketState::Status;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x01; // TODO: Check against current datagen.
}

impl PacketDecode for C2SStatusPingPacket {
    type Error = IncompleteDecodeError;

    #[inline]
    fn decode(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    { Ok(Self { timestamp : buf.read_decode()? }) }
}
