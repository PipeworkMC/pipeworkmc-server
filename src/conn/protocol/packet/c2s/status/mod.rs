use crate::conn::protocol::codec::{
    decode::{
        PacketDecode,
        PrefixedPacketDecode,
        DecodeBuf,
        IncompleteDecodeError
    },
    meta::PacketMeta
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
        Ok(match (buf.read().map_err(C2SStatusDecodeError::Incomplete)?) {
            request ::C2SStatusRequestPacket ::PREFIX => Self::Request (request ::C2SStatusRequestPacket ::decode(buf)?),
            ping    ::C2SStatusPingPacket    ::PREFIX => Self::Ping    (ping    ::C2SStatusPingPacket    ::decode(buf).map_err(C2SStatusDecodeError::Ping)?),

            v => { return Err(C2SStatusDecodeError::UnknownPrefix(v)); }
        })
    }
}


#[derive(Debug)]
pub enum C2SStatusDecodeError {
    Incomplete(IncompleteDecodeError),
    Ping (IncompleteDecodeError),
    UnknownPrefix(u8)
}
impl From<!> for C2SStatusDecodeError {
    #[inline(always)]
    fn from(_ : !) -> Self { unsafe { unreachable_unchecked() } }
}
impl Display for C2SStatusDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Incomplete(err)   => err.fmt(f),
        Self::Ping (err)        => write!(f, "ping {err}"),
        Self::UnknownPrefix (b) => write!(f, "unknown prefix `0x{b:0>2X}`"),
    } }
}
