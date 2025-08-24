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


pub mod response;
pub mod pong;


#[derive(Debug)]
pub enum S2CStatusPackets<'l> {
    Response (response ::S2CStatusResponsePacket<'l>),
    Pong     (pong     ::S2CStatusPongPacket)
}

impl S2CStatusPackets<'_> {

    pub fn prefix(&self) -> u8 { match (self) {
        Self::Response (_) => response ::S2CStatusResponsePacket ::PREFIX,
        Self::Pong     (_) => pong     ::S2CStatusPongPacket     ::PREFIX
    } }

}

unsafe impl PrefixedPacketEncode for S2CStatusPackets<'_> {

    fn encode_prefixed_len(&self) -> usize { match (self) {
        Self::Response (packet) => packet.encode_prefixed_len(),
        Self::Pong     (packet) => packet.encode_prefixed_len()
    } }

    unsafe fn encode_prefixed(&self, buf : &mut EncodeBuf) { unsafe { match (self) {
        Self::Response (packet) => packet.encode_prefixed(buf),
        Self::Pong     (packet) => packet.encode_prefixed(buf)
    } } }

}

impl<'l> From<S2CStatusPackets<'l>> for S2CPackets<'l> {
    #[inline(always)]
    fn from(value : S2CStatusPackets<'l>) -> Self { Self::Status(value) }
}
