use crate::conn::protocol::{
    codec::decode::{
        PacketDecode,
        DecodeBuf,
        vec::VecDecodeError
    },
    packet::{
        PacketMeta,
        PacketState,
        PacketBound
    },
    value::ident::{
        Ident,
        IdentDecodeError
    }
};
use core::fmt::{ self, Display, Formatter };


#[derive(Debug)]
pub struct C2SConfigCustomPayloadPacket {
    pub channel : Ident,
    pub data    : Vec<u8>
}

impl PacketMeta for C2SConfigCustomPayloadPacket {
    const STATE  : PacketState = PacketState::Config;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x02; // TODO: Check against current datagen.
}

impl PacketDecode for C2SConfigCustomPayloadPacket {
    type Error = C2SConfigCustomPayloadDecodeError;

    fn decode(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    { Ok(Self {
        channel : <_>::decode(buf).map_err(C2SConfigCustomPayloadDecodeError::Channel)?,
        data    : <Vec<u8>>::decode(buf).map_err(C2SConfigCustomPayloadDecodeError::Data)?
    }) }
}


#[derive(Debug)]
pub enum C2SConfigCustomPayloadDecodeError {
    Channel(IdentDecodeError),
    Data(VecDecodeError<<u8 as PacketDecode>::Error>)
}
impl Display for C2SConfigCustomPayloadDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Channel(err) => write!(f, "channel {err}"),
        Self::Data(err)    => write!(f, "data {err}")
    } }
}
