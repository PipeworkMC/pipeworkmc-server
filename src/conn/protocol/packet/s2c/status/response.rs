use crate::conn::protocol::{
    codec::encode::{
        PacketEncode,
        EncodeBuf
    },
    packet::{
        PacketMeta,
        PacketState,
        PacketBound,
        s2c::{
            S2CPackets,
            status::S2CStatusPackets
        }
    }
};
use std::borrow::Cow;
use serde::Serialize as Ser;
use serde_json::to_string as to_json_string;
use uuid::Uuid;


#[derive(Debug)]
pub struct S2CStatusResponsePacket {
    status_json : String
}

impl PacketMeta for S2CStatusResponsePacket {
    const STATE  : PacketState = PacketState::Status;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x00; // TODO: Check against current datagen.
}

unsafe impl PacketEncode for S2CStatusResponsePacket {

    #[inline(always)]
    fn encode_len(&self) -> usize {
        self.status_json.encode_len()
    }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        buf.encode_write(&self.status_json);
    } }

}

impl From<S2CStatusResponsePacket> for S2CPackets {
    #[inline(always)]
    fn from(value : S2CStatusResponsePacket) -> Self { Self::Status(value.into()) }
}

impl From<S2CStatusResponsePacket> for S2CStatusPackets {
    #[inline(always)]
    fn from(value : S2CStatusResponsePacket) -> Self { Self::Response(value) }
}


#[derive(Ser)]
pub struct Status {
    pub version               : StatusVersion,
    pub players               : Option<StatusPlayers>,
    #[serde(rename = "description")]
    pub motd                  : Option<()>, // TODO
    pub favicon               : Cow<'static, str>,
    #[serde(rename = "enforcesSecureChat")]
    pub enforces_secure_chat  : bool,
    #[serde(rename = "preventsChatReports")]
    pub prevents_chat_reports : bool
}

#[derive(Ser)]
pub struct StatusVersion {
    pub name     : Option<Cow<'static, str>>,
    pub protocol : u32
}

#[derive(Ser)]
pub struct StatusPlayers {
    #[serde(rename = "online")]
    pub current : u32,
    pub max     : u32,
    pub sample  : Cow<'static, [StatusPlayer]>
}

#[derive(Ser, Clone)]
pub struct StatusPlayer {
    #[serde(rename = "id")]
    uuid : Uuid,
    name : String
}

impl From<Status> for S2CStatusResponsePacket {
    #[inline]
    fn from(value : Status) -> Self {
        Self { status_json : to_json_string(&value).unwrap() }
    }
}
