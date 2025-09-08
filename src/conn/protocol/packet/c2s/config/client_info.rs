use crate::conn::protocol::{
    codec::{
        decode::{
            PacketDecode,
            DecodeBuf
        },
        meta::{
            PacketMeta,
            PacketState,
            PacketBound
        }
    }
};
use crate::data::client_info::{
    ClientInfo,
    ClientInfoDecodeError
};


#[derive(Debug)]
pub struct C2SConfigClientInfoPacket {
    pub info : ClientInfo
}

impl PacketMeta for C2SConfigClientInfoPacket {
    const STATE  : PacketState = PacketState::Config;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x00; // TODO: Check against current datagen.
}

impl PacketDecode for C2SConfigClientInfoPacket {
    type Error = ClientInfoDecodeError;

    fn decode(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    { Ok(Self {
        info : <_>::decode(buf)?
    }) }
}
