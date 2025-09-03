use crate::conn::protocol::{
    codec::decode::{
        PacketDecode,
        PrefixedPacketDecode,
        DecodeBuf,
        IncompleteDecodeError
    },
    packet::PacketMeta,
    value::{
        client_info::ClientInfoDecodeError,
        channel_data::ChannelDataDecodeError
    }
};
use core::{
    fmt::{ self, Display, Formatter },
    hint::unreachable_unchecked
};


pub mod client_info;
pub mod custom_payload;
pub mod finish_acknowledged;


#[derive(Debug)]
pub enum C2SConfigPackets {
    ClientInfo         (client_info  ::C2SConfigClientInfoPacket),
    // TODO: CookieResponse
    CustomPayload      (custom_payload      ::C2SConfigCustomPayloadPacket),
    FinishAcknowledged (finish_acknowledged ::C2SConfigFinishAcknowledgedPacket)
    // TODO: Keepalive
    // TODO: Pong
    // TODO: ResourcePack
    // TODO: SelectKnownPacks
    // TODO: CustomClickAction
}

impl PrefixedPacketDecode for C2SConfigPackets {
    type Error = C2SConfigDecodeError;

    fn decode_prefixed(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    {
        Ok(match (buf.read().map_err(C2SConfigDecodeError::Incomplete)?) {
            client_info         ::C2SConfigClientInfoPacket         ::PREFIX => Self::ClientInfo         (client_info         ::C2SConfigClientInfoPacket         ::decode(buf).map_err(C2SConfigDecodeError::ClientInfo)?),
            custom_payload      ::C2SConfigCustomPayloadPacket      ::PREFIX => Self::CustomPayload      (custom_payload      ::C2SConfigCustomPayloadPacket      ::decode(buf).map_err(C2SConfigDecodeError::CustomPayload)?),
            finish_acknowledged ::C2SConfigFinishAcknowledgedPacket ::PREFIX => Self::FinishAcknowledged (finish_acknowledged ::C2SConfigFinishAcknowledgedPacket ::decode(buf)?),

            v => { return Err(C2SConfigDecodeError::UnknownPrefix(v)); }
        })
    }
}


#[derive(Debug)]
pub enum C2SConfigDecodeError {
    Incomplete(IncompleteDecodeError),
    ClientInfo         (ClientInfoDecodeError),
    CustomPayload      (ChannelDataDecodeError),
    UnknownPrefix(u8)
}
impl From<!> for C2SConfigDecodeError {
    #[inline(always)]
    fn from(_ : !) -> Self { unsafe { unreachable_unchecked() } }
}
impl Display for C2SConfigDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Incomplete(err)    => err.fmt(f),
        Self::ClientInfo(err)    => write!(f, "client info {err}"),
        Self::CustomPayload(err) => write!(f, "custom payload {err}"),
        Self::UnknownPrefix(b)   => write!(f, "unknown prefix `0x{b:0>2x}`"),
    } }
}
