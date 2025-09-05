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
    }
};
use crate::data::redacted::Redacted;
use core::fmt::{ self, Display, Formatter };


#[derive(Debug)]
pub struct C2SLoginEncryptResponsePacket {
    pub encrypted_secret_key : Redacted<Vec<u8>>,
    pub encrypted_vtoken     : Vec<u8>
}

impl PacketMeta for C2SLoginEncryptResponsePacket {
    const STATE  : PacketState = PacketState::Login;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x01; // TODO: Check against current datagen.
}

impl PacketDecode for C2SLoginEncryptResponsePacket {
    type Error = C2SLoginEncryptResponseDecodeError;

    #[inline]
    fn decode(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    { Ok(Self {
        encrypted_secret_key : Redacted::from(<Vec<u8>>::decode(buf).map_err(C2SLoginEncryptResponseDecodeError::SecretKey)?), // TODO: Validate username characters
        encrypted_vtoken     : <_>::decode(buf).map_err(C2SLoginEncryptResponseDecodeError::VerifyToken)?
    }) }
}


#[derive(Debug)]
pub enum C2SLoginEncryptResponseDecodeError {
    SecretKey(VecDecodeError<<u8 as PacketDecode>::Error>),
    VerifyToken(VecDecodeError<<u8 as PacketDecode>::Error>)
}
impl Display for C2SLoginEncryptResponseDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::SecretKey(err)   => write!(f, "secret key {err}"),
        Self::VerifyToken(err) => write!(f, "verify token {err}")
    } }
}
