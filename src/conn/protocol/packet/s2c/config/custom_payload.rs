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
            config::S2CConfigPackets
        }
    },
    value::channel_data::ChannelData
};


#[derive(Debug)]
pub struct S2CConfigCustomPayloadPacket<'l> {
    pub data : ChannelData<'l>
}


impl PacketMeta for S2CConfigCustomPayloadPacket<'_> {
    const STATE  : PacketState = PacketState::Config;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x01; // TODO: Check against current datagen.
}

unsafe impl PacketEncode for S2CConfigCustomPayloadPacket<'_> {

    #[inline(always)]
    fn encode_len(&self) -> usize {
        self.data.encode_len()
    }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.data.encode(buf);
    } }

}

impl<'l> From<S2CConfigCustomPayloadPacket<'l>> for S2CPackets<'l> {
    #[inline(always)]
    fn from(value : S2CConfigCustomPayloadPacket<'l>) -> Self { Self::Config(value.into()) }
}

impl<'l> From<S2CConfigCustomPayloadPacket<'l>> for S2CConfigPackets<'l> {
    #[inline(always)]
    fn from(value : S2CConfigCustomPayloadPacket<'l>) -> Self { Self::CustomPayload(value) }
}
