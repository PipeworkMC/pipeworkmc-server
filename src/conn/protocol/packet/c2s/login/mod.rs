use crate::conn::protocol::{
    codec::decode::{
        PacketDecode,
        PrefixedPacketDecode,
        DecodeBuf,
        IncompleteDecodeError
    },
    packet::PacketMeta
};


pub mod start;


#[derive(Debug)]
pub enum C2SLoginPackets {
    Start(start::C2SLoginStartPacket)
}

impl PrefixedPacketDecode for C2SLoginPackets {
    type Error = C2SLoginDecodeError;

    fn decode_prefixed(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    {
        Ok(match (buf.read()?) {
            start::C2SLoginStartPacket ::PREFIX => Self::Start(start::C2SLoginStartPacket::decode(buf)?),

            v => { return Err(C2SLoginDecodeError::UnknownPrefix(v)); }
        })
    }
}


#[derive(Debug)]
pub enum C2SLoginDecodeError {
    Incomplete,
    Start(start::C2SLoginStartDecodeError),
    UnknownPrefix(u8)
}
impl From<IncompleteDecodeError> for C2SLoginDecodeError {
    #[inline(always)]
    fn from(_ : IncompleteDecodeError) -> Self { Self::Incomplete }
}
impl From<start::C2SLoginStartDecodeError> for C2SLoginDecodeError {
    #[inline(always)]
    fn from(value : start::C2SLoginStartDecodeError) -> Self { Self::Start(value) }
}
