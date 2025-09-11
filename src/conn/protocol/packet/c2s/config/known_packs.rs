use crate::conn::protocol::{
    codec::{
        decode::{
            PacketDecode,
            DecodeBuf,
            vec::VecDecodeError
        },
        meta::{
            PacketMeta,
            PacketState,
            PacketBound
        }
    }
};
use crate::data::known_pack::{
    KnownPack,
    KnownPackDecodeError
};


#[derive(Debug)]
pub struct C2SConfigKnownPacksPacket {
    pub known_packs : Vec<KnownPack<'static>>
}

impl PacketMeta for C2SConfigKnownPacksPacket {
    const STATE  : PacketState = PacketState::Config;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x07; // TODO: Check against current datagen.
}

impl PacketDecode for C2SConfigKnownPacksPacket {
    type Error = VecDecodeError<KnownPackDecodeError>;

    fn decode(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    { Ok(Self {
        known_packs : <_>::decode(buf)?,
    }) }
}
