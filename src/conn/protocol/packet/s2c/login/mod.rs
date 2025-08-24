use crate::conn::protocol::{
    codec::encode::{
        PrefixedPacketEncode,
        EncodeBuf
    },
    packet::s2c::S2CPackets
};


pub mod encrypt_request;
pub mod success;
pub mod compression;


#[derive(Debug)]
pub enum S2CLoginPackets {
    EncryptRequest (encrypt_request ::S2CLoginEncryptRequestPacket),
    Success        (success         ::S2CLoginSuccessPacket),
    Compression    (compression     ::S2CLoginCompressionPacket)
}

unsafe impl PrefixedPacketEncode for S2CLoginPackets {

    fn encode_prefixed_len(&self) -> usize { match (self) {
        Self::EncryptRequest (packet) => packet.encode_prefixed_len(),
        Self::Success        (packet) => packet.encode_prefixed_len(),
        Self::Compression    (packet) => packet.encode_prefixed_len()
    } }

    unsafe fn encode_prefixed(&self, buf : &mut EncodeBuf) { unsafe { match (self) {
        Self::EncryptRequest (packet) => packet.encode_prefixed(buf),
        Self::Success        (packet) => packet.encode_prefixed(buf),
        Self::Compression    (packet) => packet.encode_prefixed(buf)
    } } }

}

impl From<S2CLoginPackets> for S2CPackets {
    #[inline(always)]
    fn from(value : S2CLoginPackets) -> Self { Self::Login(value) }
}
