use crate::conn::protocol::{
    codec::encode::{
        PrefixedPacketEncode,
        EncodeBuf
    },
    packet::s2c::S2CPackets
};


pub mod response;
pub mod pong;


#[derive(Debug)]
pub enum S2CStatusPackets {
    Response (response ::S2CStatusResponsePacket),
    Pong     (pong     ::S2CStatusPongPacket)
}

unsafe impl PrefixedPacketEncode for S2CStatusPackets {

    fn encode_prefixed_len(&self) -> usize { match (self) {
        Self::Response (packet) => packet.encode_prefixed_len(),
        Self::Pong     (packet) => packet.encode_prefixed_len()
    } }

    unsafe fn encode_prefixed(&self, buf : &mut EncodeBuf) { unsafe { match (self) {
        Self::Response (packet) => packet.encode_prefixed(buf),
        Self::Pong     (packet) => packet.encode_prefixed(buf)
    } } }

}

impl From<S2CStatusPackets> for S2CPackets {
    #[inline(always)]
    fn from(value : S2CStatusPackets) -> Self { Self::Status(value) }
}
