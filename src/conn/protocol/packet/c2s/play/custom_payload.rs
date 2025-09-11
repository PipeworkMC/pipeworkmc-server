use crate::conn::protocol::{
    codec::{
        decode::{
            PacketDecode,
            DecodeBuf
        },
        meta::{
            PacketMeta,
            PacketState,
            PacketBound
        }
    }
};
use crate::data::channel_data::{
    ChannelData,
    ChannelDataDecodeError
};


#[derive(Debug)]
pub struct C2SPlayCustomPayloadPacket {
    pub data : ChannelData<'static>
}

impl PacketMeta for C2SPlayCustomPayloadPacket {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x15; // TODO: Check against current datagen.
}

impl PacketDecode for C2SPlayCustomPayloadPacket {
    type Error = ChannelDataDecodeError;

    fn decode(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    { Ok(Self {
        data : <_>::decode(buf)?,
    }) }
}
