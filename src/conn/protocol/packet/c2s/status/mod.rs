use crate::conn::protocol::{
    codec::decode::{
        PacketDecode,
        PrefixedPacketDecode,
        DecodeBuf,
        IncompleteDecodeError
    },
    packet::PacketMeta
};
use core::{
    fmt::{ self, Display, Formatter },
    hint::unreachable_unchecked
};


pub mod request;
pub mod ping;


#[derive(Debug)]
pub enum C2SStatusPackets {
    Request (request ::C2SStatusRequestPacket),
    Ping    (ping    ::C2SStatusPingPacket)
}

impl PrefixedPacketDecode for C2SStatusPackets {
    type Error = C2SStatusDecodeError;

    fn decode_prefixed(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    {
        Ok(match (buf.read()?) {
            request ::C2SStatusRequestPacket ::PREFIX => Self::Request (request ::C2SStatusRequestPacket ::decode(buf)?),
            ping    ::C2SStatusPingPacket    ::PREFIX => Self::Ping    (ping    ::C2SStatusPingPacket    ::decode(buf)?),

            v => { return Err(C2SStatusDecodeError::UnknownPrefix(v)); }
        })
    }
}


#[derive(Debug)]
pub enum C2SStatusDecodeError {
    Incomplete(IncompleteDecodeError),
    UnknownPrefix(u8)
}
impl From<!> for C2SStatusDecodeError {
    #[inline(always)]
    fn from(_ : !) -> Self { unsafe { unreachable_unchecked() } }
}
impl From<IncompleteDecodeError> for C2SStatusDecodeError {
    #[inline(always)]
    fn from(err : IncompleteDecodeError) -> Self { Self::Incomplete(err) }
}
impl Display for C2SStatusDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Incomplete(err)   => err.fmt(f),
        Self::UnknownPrefix (b) => write!(f, "unknown prefix `{b:0>2b}`"),
    } }
}
