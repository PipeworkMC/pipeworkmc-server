use crate::conn::protocol::{
    codec::encode::{
        PrefixedPacketEncode,
        EncodeBuf
    },
    packet::{
        PacketMeta,
        s2c::S2CPackets
    }
};


pub mod disconnect;


#[derive(Debug)]
pub enum S2CPlayPackets {
    Disconnect (disconnect ::S2CPlayDisconnectPacket)
}

impl S2CPlayPackets {

    pub fn prefix(&self) -> u8 { match (self) {
        Self::Disconnect (_) => disconnect ::S2CPlayDisconnectPacket ::PREFIX
    } }

}

unsafe impl PrefixedPacketEncode for S2CPlayPackets {

    fn encode_prefixed_len(&self) -> usize { match (self) {
        Self::Disconnect (packet) => packet.encode_prefixed_len()
    } }

    unsafe fn encode_prefixed(&self, buf : &mut EncodeBuf) { unsafe { match (self) {
        Self::Disconnect (packet) => packet.encode_prefixed(buf)
    } } }

}

impl<'l> From<S2CPlayPackets> for S2CPackets<'l> {
    #[inline(always)]
    fn from(value : S2CPlayPackets) -> Self { Self::Play(value) }
}
