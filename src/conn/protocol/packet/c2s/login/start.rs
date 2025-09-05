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
use crate::data::bounded_string::{
    BoundedString,
    BoundedStringDecodeError
};
use core::fmt::{ self, Display, Formatter };
use uuid::Uuid;


#[derive(Debug)]
pub struct C2SLoginStartPacket {
    pub username : BoundedString<16>,
    pub uuid     : Uuid
}

impl PacketMeta for C2SLoginStartPacket {
    const STATE  : PacketState = PacketState::Login;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x00; // TODO: Check against current datagen.
}

impl PacketDecode for C2SLoginStartPacket {
    type Error = C2SLoginStartDecodeError;

    #[inline]
    fn decode(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    { Ok(Self {
        username : <_>::decode(buf).map_err(C2SLoginStartDecodeError::Username)?, // TODO: Validate username characters
        uuid     : <_>::decode(buf).map_err(C2SLoginStartDecodeError::Uuid)?
    }) }
}


#[derive(Debug)]
pub enum C2SLoginStartDecodeError {
    Username(BoundedStringDecodeError),
    Uuid(IncompleteDecodeError)
}
impl Display for C2SLoginStartDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Username(err) => write!(f, "username {err}"),
        Self::Uuid(err)     => write!(f, "uuid {err}")
    } }
}
