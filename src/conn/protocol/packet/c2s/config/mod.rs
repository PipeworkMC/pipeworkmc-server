use crate::conn::protocol::codec::{
    decode::{
        PacketDecode,
        PrefixedPacketDecode,
        DecodeBuf,
        IncompleteDecodeError,
        vec::VecDecodeError
    },
    meta::PacketMeta
};
use crate::data::{
    client_info::ClientInfoDecodeError,
    channel_data::ChannelDataDecodeError,
    known_pack::KnownPackDecodeError
};
use core::{
    fmt::{ self, Display, Formatter },
    hint::unreachable_unchecked
};


pub mod client_info;
pub mod custom_payload;
pub mod finish_acknowledged;
pub mod keep_alive;
pub mod known_packs;


#[derive(Debug)]
pub enum C2SConfigPackets {
    ClientInfo         (client_info         ::C2SConfigClientInfoPacket),
    // TODO: CookieResponse
    CustomPayload      (custom_payload      ::C2SConfigCustomPayloadPacket),
    FinishAcknowledged (finish_acknowledged ::C2SConfigFinishAcknowledgedPacket),
    KeepAlive          (keep_alive          ::C2SConfigKeepAlivePacket),
    // TODO: Pong
    // TODO: ResourcePack
    KnownPacks         (known_packs         ::C2SConfigKnownPacksPacket)
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
            keep_alive          ::C2SConfigKeepAlivePacket          ::PREFIX => Self::KeepAlive          (keep_alive          ::C2SConfigKeepAlivePacket          ::decode(buf).map_err(C2SConfigDecodeError::KeepAlive)?),
            known_packs         ::C2SConfigKnownPacksPacket         ::PREFIX => Self::KnownPacks         (known_packs         ::C2SConfigKnownPacksPacket         ::decode(buf).map_err(C2SConfigDecodeError::KnownPacks)?),

            v => { return Err(C2SConfigDecodeError::UnknownPrefix(v)); }
        })
    }
}


#[derive(Debug)]
pub enum C2SConfigDecodeError {
    Incomplete(IncompleteDecodeError),
    ClientInfo    (ClientInfoDecodeError),
    CustomPayload (ChannelDataDecodeError),
    KeepAlive     (IncompleteDecodeError),
    KnownPacks    (VecDecodeError<KnownPackDecodeError>),
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
        Self::KeepAlive(err)     => write!(f, "keep alive {err}"),
        Self::KnownPacks(err)    => write!(f, "known packs {err}"),
        Self::UnknownPrefix(b)   => write!(f, "unknown prefix `0x{b:0>2X}`"),
    } }
}
