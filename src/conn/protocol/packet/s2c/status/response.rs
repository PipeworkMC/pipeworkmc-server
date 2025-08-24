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
    },
    value::text::{
        Text,
        TextComponent,
        TextContent
    },
    Protocol
};
use std::borrow::Cow;
use serde::{
    Serialize as Ser,
    Serializer as Serer
};
use serde_json::to_string as to_json_string;
use uuid::Uuid;


#[derive(Debug)]
pub struct S2CStatusResponsePacket {
    status_json : Cow<'static, str>
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
        self.status_json.encode(buf);
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub players               : Option<StatusPlayers>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub motd                  : Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "add_png_b64_header")]
    pub favicon               : Option<Cow<'static, str>>,
    #[serde(rename = "enforcesSecureChat")]
    pub enforces_secure_chat  : bool,
    #[serde(rename = "preventsChatReports")]
    pub prevents_chat_reports : bool
}
fn add_png_b64_header<S : Serer>(png_b64 : &Option<Cow<'static, str>>, ser : S) -> Result<S::Ok, S::Error> {
    if let Some(png_b64) = png_b64 {
        ser.serialize_str(&format!("data:image/png;base64,{png_b64}"))
    } else {
        ser.serialize_none()
    }
}

#[derive(Ser)]
pub struct StatusVersion {
    pub name     : Cow<'static, str>,
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
    pub uuid : Uuid,
    pub name : String
}


impl Default for Status {
    fn default() -> Self { Self {
        version               : StatusVersion::default(),
        players               : None,
        motd                  : Some(Text { components : Cow::Borrowed(&[TextComponent {
            content : TextContent::Literal { text : Cow::Borrowed("A PipeworkMC Server") },
            .. TextComponent::EMPTY
        }]) }),
        favicon               : None,
        enforces_secure_chat  : false,
        prevents_chat_reports : true
    } }
}

impl Default for StatusVersion {
    #[inline]
    fn default() -> Self { Self {
        name     : Cow::Borrowed(Protocol::LATEST.earliest_name()),
        protocol : Protocol::LATEST.id()
    } }
}


impl From<&Status> for S2CStatusResponsePacket {
    #[inline]
    fn from(value : &Status) -> Self {
        Self { status_json : Cow::Owned(to_json_string(&value).unwrap()) }
    }
}
impl From<Status> for S2CStatusResponsePacket {
    #[inline]
    fn from(value : Status) -> Self {
        Self { status_json : Cow::Owned(to_json_string(&value).unwrap()) }
    }
}
