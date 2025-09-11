use crate::conn::protocol::{
    codec::{
        decode::{
            PacketDecode,
            DecodeBuf,
            IncompleteDecodeError
        },
        meta::{
            PacketMeta,
            PacketState,
            PacketBound
        }
    }
};


#[derive(Debug)]
pub struct C2SConfigKeepAlivePacket {
    pub id : u64
}

impl PacketMeta for C2SConfigKeepAlivePacket {
    const STATE  : PacketState = PacketState::Config;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x04; // TODO: Check against current datagen.
}

impl PacketDecode for C2SConfigKeepAlivePacket {
    type Error = IncompleteDecodeError;

    #[inline(always)]
    fn decode(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    { Ok(Self {
        id : <_>::decode(buf)?
    }) }
}
