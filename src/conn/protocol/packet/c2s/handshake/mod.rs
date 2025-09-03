use crate::conn::protocol::{
    codec::decode::{
        PacketDecode,
        PrefixedPacketDecode,
        DecodeBuf,
        IncompleteDecodeError
    },
    packet::PacketMeta
};
use core::fmt::{ self, Display, Formatter };


pub mod intention;


#[derive(Debug)]
pub enum C2SHandshakePackets {
    Intention(intention::C2SHandshakeIntentionPacket)
}

impl PrefixedPacketDecode for C2SHandshakePackets {
    type Error = C2SHandshakeDecodeError;

    fn decode_prefixed(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    {
        Ok(match (buf.read().map_err(C2SHandshakeDecodeError::Incomplete)?) {
            intention::C2SHandshakeIntentionPacket::PREFIX => Self::Intention(intention::C2SHandshakeIntentionPacket::decode(buf).map_err(C2SHandshakeDecodeError::Intention)?),

            v => { return Err(C2SHandshakeDecodeError::UnknownPrefix(v)); }
        })
    }
}


#[derive(Debug)]
pub enum C2SHandshakeDecodeError {
    Incomplete(IncompleteDecodeError),
    Intention(intention::C2SHandshakeIntentionDecodeError),
    UnknownPrefix(u8)
}
impl Display for C2SHandshakeDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Incomplete(err)   => err.fmt(f),
        Self::Intention(err)    => write!(f, "intention {err}"),
        Self::UnknownPrefix (b) => write!(f, "unknown prefix `0x{b:0>2x}`"),
    } }
}
