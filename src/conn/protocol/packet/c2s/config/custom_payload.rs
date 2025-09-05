use crate::conn::protocol::{
    codec::decode::{
        PacketDecode,
        DecodeBuf
    },
    packet::{
        PacketMeta,
        PacketState,
        PacketBound
    }
};
use crate::data::channel_data::{
    ChannelData,
    ChannelDataDecodeError
};


#[derive(Debug)]
pub struct C2SConfigCustomPayloadPacket {
    pub data : ChannelData<'static>
}

impl PacketMeta for C2SConfigCustomPayloadPacket {
    const STATE  : PacketState = PacketState::Config;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x02; // TODO: Check against current datagen.
}

impl PacketDecode for C2SConfigCustomPayloadPacket {
    type Error = ChannelDataDecodeError;

    fn decode(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    { Ok(Self {
        data : <_>::decode(buf)?,
    }) }
}
