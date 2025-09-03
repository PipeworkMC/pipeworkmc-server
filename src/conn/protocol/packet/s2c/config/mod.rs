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
pub mod finish;


#[derive(Debug)]
pub enum S2CConfigPackets {
    Disconnect (disconnect ::S2CConfigDisconnectPacket),
    Finish     (finish     ::S2CConfigFinishPacket)
}

impl S2CConfigPackets {

    pub fn prefix(&self) -> u8 { match (self) {
        Self::Disconnect (_) => disconnect ::S2CConfigDisconnectPacket ::PREFIX,
        Self::Finish     (_) => finish     ::S2CConfigFinishPacket      ::PREFIX
    } }

}

unsafe impl PrefixedPacketEncode for S2CConfigPackets {

    fn encode_prefixed_len(&self) -> usize { match (self) {
        Self::Disconnect (packet) => packet.encode_prefixed_len(),
        Self::Finish     (packet) => packet.encode_prefixed_len()
    } }

    unsafe fn encode_prefixed(&self, buf : &mut EncodeBuf) { unsafe { match (self) {
        Self::Disconnect (packet) => packet.encode_prefixed(buf),
        Self::Finish     (packet) => packet.encode_prefixed(buf)
    } } }

}

impl<'l> From<S2CConfigPackets> for S2CPackets<'l> {
    #[inline(always)]
    fn from(value : S2CConfigPackets) -> Self { Self::Config(value) }
}
