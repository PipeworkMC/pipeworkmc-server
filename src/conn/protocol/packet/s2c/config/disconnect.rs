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
    value::{
        text::Text,
        nbt::to_network as to_network_nbt
    }
};


#[derive(Debug)]
pub struct S2CConfigDisconnectPacket {
    reason_nbt : Vec<u8>
}

impl PacketMeta for S2CConfigDisconnectPacket {
    const STATE  : PacketState = PacketState::Config;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x02; // TODO: Check against current datagen.
}

unsafe impl PacketEncode for S2CConfigDisconnectPacket {

    #[inline(always)]
    fn encode_len(&self) -> usize {
        self.reason_nbt.len()
    }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        buf.write_slice(&self.reason_nbt);
    } }

}

impl From<S2CConfigDisconnectPacket> for S2CPackets<'_> {
    #[inline(always)]
    fn from(value : S2CConfigDisconnectPacket) -> Self { Self::Config(value.into()) }
}

impl From<S2CConfigDisconnectPacket> for S2CConfigPackets {
    #[inline(always)]
    fn from(value : S2CConfigDisconnectPacket) -> Self { Self::Disconnect(value) }
}


impl<S> From<S> for S2CConfigDisconnectPacket
where
    S : Into<Text>
{
    #[inline]
    fn from(value : S) -> Self {
        let mut reason_nbt = Vec::new();
        to_network_nbt(&mut reason_nbt, &value.into()).unwrap();
        Self { reason_nbt }
    }
}
