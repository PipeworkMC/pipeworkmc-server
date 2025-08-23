use crate::conn::protocol::{
    codec::decode::{
        PacketDecode,
        DecodeBuf,
        IncompleteDecodeError,
        string::StringDecodeError
    },
    packet::{
        PacketMeta,
        PacketState,
        PacketBound
    },
    value::varint::{
        VarInt,
        VarIntDecodeError
    }
};


#[derive(Debug)]
pub struct C2SHandshakeIntentionPacket {
    pub protocol       : VarInt<u32>,
    pub server_address : String,
    pub server_port    : u16,
    pub intent         : Intention
}

impl PacketMeta for C2SHandshakeIntentionPacket {
    const STATE  : PacketState = PacketState::Handshake;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x00; // TODO: Check against current datagen.
}

impl PacketDecode for C2SHandshakeIntentionPacket {
    type Error = IntentionDecodeError;

    fn decode(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    { Ok(Self {
        protocol       : buf.read_decode().map_err(IntentionDecodeError::Protocol)?,
        server_address : buf.read_decode().map_err(IntentionDecodeError::Address)?,
        server_port    : buf.read_decode().map_err(IntentionDecodeError::Port)?,
        intent         : match (*buf.read_decode::<VarInt<u32>>().map_err(IntentionDecodeError::Intent)?) {
            1 => Intention::Status,
            2 => Intention::Login { is_transfer : false },
            3 => Intention::Login { is_transfer : true },
            v => { return Err(IntentionDecodeError::UnknownIntention(v)); }
        }
    }) }
}


#[derive(Debug)]
pub enum Intention {
    Status,
    Login {
        is_transfer : bool
    }
}


#[derive(Debug)]
pub enum IntentionDecodeError {
    Protocol(VarIntDecodeError),
    Address(StringDecodeError),
    Port(IncompleteDecodeError),
    Intent(VarIntDecodeError),
    UnknownIntention(u32)
}
