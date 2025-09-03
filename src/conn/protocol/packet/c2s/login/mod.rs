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


pub mod start;
pub mod encrypt_response;
pub mod finish_acknowledged;


#[derive(Debug)]
pub enum C2SLoginPackets {
    Start              (start               ::C2SLoginStartPacket),
    EncryptResponse    (encrypt_response    ::C2SLoginEncryptResponsePacket),
    // TODO: QueryResponse
    FinishAcknowledged (finish_acknowledged ::C2SLoginFinishAcknowledgedPacket)
    // TODO: Cookie response
}

impl PrefixedPacketDecode for C2SLoginPackets {
    type Error = C2SLoginDecodeError;

    fn decode_prefixed(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    {
        Ok(match (buf.read()?) {
            start               ::C2SLoginStartPacket              ::PREFIX => Self::Start              (start               ::C2SLoginStartPacket              ::decode(buf)?),
            encrypt_response    ::C2SLoginEncryptResponsePacket    ::PREFIX => Self::EncryptResponse    (encrypt_response    ::C2SLoginEncryptResponsePacket    ::decode(buf)?),
            finish_acknowledged ::C2SLoginFinishAcknowledgedPacket ::PREFIX => Self::FinishAcknowledged (finish_acknowledged ::C2SLoginFinishAcknowledgedPacket ::decode(buf)?),

            v => { return Err(C2SLoginDecodeError::UnknownPrefix(v)); }
        })
    }
}


#[derive(Debug)]
pub enum C2SLoginDecodeError {
    Incomplete(IncompleteDecodeError),
    Start(start::C2SLoginStartDecodeError),
    EncryptResponse(encrypt_response::C2SLoginEncryptResponseDecodeError),
    UnknownPrefix(u8)
}
impl From<!> for C2SLoginDecodeError {
    #[inline(always)]
    fn from(_ : !) -> Self { unsafe { unreachable_unchecked() } }
}
impl From<IncompleteDecodeError> for C2SLoginDecodeError {
    #[inline(always)]
    fn from(err : IncompleteDecodeError) -> Self { Self::Incomplete(err) }
}
impl From<start::C2SLoginStartDecodeError> for C2SLoginDecodeError {
    #[inline(always)]
    fn from(value : start::C2SLoginStartDecodeError) -> Self { Self::Start(value) }
}
impl From<encrypt_response::C2SLoginEncryptResponseDecodeError> for C2SLoginDecodeError {
    #[inline(always)]
    fn from(value : encrypt_response::C2SLoginEncryptResponseDecodeError) -> Self { Self::EncryptResponse(value) }
}
impl Display for C2SLoginDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Incomplete(err)      => err.fmt(f),
        Self::Start(err)           => write!(f, "start {err}"),
        Self::EncryptResponse(err) => write!(f, "encrypt response {err}"),
        Self::UnknownPrefix (b)    => write!(f, "unknown prefix `0x{b:0>2x}`"),
    } }
}
