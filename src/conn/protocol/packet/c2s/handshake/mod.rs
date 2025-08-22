use crate::conn::protocol::{
    codec::decode::{
        PacketDecode,
        PrefixedPacketDecode,
        DecodeBuf,
        IncompleteDecodeError
    },
    packet::PacketMeta
};


pub mod intention;


#[derive(Debug)]
pub enum C2SHandshakePackets {
    Intention(intention::IntentionPacket)
}

impl PrefixedPacketDecode for C2SHandshakePackets {
    type Error = C2SHandshakeDecodeError;

    fn decode_prefixed(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    {
        Ok(match (buf.read()?) {
            intention::IntentionPacket::PREFIX => Self::Intention(intention::IntentionPacket::decode(buf).map_err(C2SHandshakeDecodeError::Intention)?),

            v => { return Err(C2SHandshakeDecodeError::UnknownPrefix(v)); }
        })
    }
}


#[derive(Debug)]
pub enum C2SHandshakeDecodeError {
    Incomplete,
    Intention(intention::IntentionDecodeError),
    UnknownPrefix(u8)
}
impl From<IncompleteDecodeError> for C2SHandshakeDecodeError {
    #[inline(always)]
    fn from(_ : IncompleteDecodeError) -> Self {
        Self::Incomplete
    }
}
