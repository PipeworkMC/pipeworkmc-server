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
            play::S2CPlayPackets
        }
    },
    value::{
        text::Text,
        nbt::to_network as to_network_nbt
    }
};


#[derive(Debug)]
pub struct S2CPlayDisconnectPacket {
    reason_nbt : Vec<u8>
}

impl PacketMeta for S2CPlayDisconnectPacket {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x2C; // TODO: Check against current datagen.
}

unsafe impl PacketEncode for S2CPlayDisconnectPacket {

    #[inline(always)]
    fn encode_len(&self) -> usize {
        self.reason_nbt.len()
    }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        buf.write_slice(&self.reason_nbt);
    } }

}

impl From<S2CPlayDisconnectPacket> for S2CPackets<'_> {
    #[inline(always)]
    fn from(value : S2CPlayDisconnectPacket) -> Self { Self::Play(value.into()) }
}

impl From<S2CPlayDisconnectPacket> for S2CPlayPackets {
    #[inline(always)]
    fn from(value : S2CPlayDisconnectPacket) -> Self { Self::Disconnect(value) }
}


impl<S> From<S> for S2CPlayDisconnectPacket
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
