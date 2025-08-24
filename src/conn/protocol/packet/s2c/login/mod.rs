use crate::conn::protocol::{
    codec::encode::{
        PrefixedPacketEncode,
        EncodeBuf
    },
    packet::s2c::S2CPackets
};


pub mod encrypt_request;
pub mod finish;
pub mod compression;


#[derive(Debug)]
pub enum S2CLoginPackets<'l> {
    // TODO: Disconnect
    EncryptRequest (encrypt_request ::S2CLoginEncryptRequestPacket<'l>),
    Finish         (finish          ::S2CLoginFinishPacket<'l>),
    Compression    (compression     ::S2CLoginCompressionPacket)
    // TODO: QueryRequest
    // TODO: CookieRequest
}

unsafe impl PrefixedPacketEncode for S2CLoginPackets<'_> {

    fn encode_prefixed_len(&self) -> usize { match (self) {
        Self::EncryptRequest (packet) => packet.encode_prefixed_len(),
        Self::Finish         (packet) => packet.encode_prefixed_len(),
        Self::Compression    (packet) => packet.encode_prefixed_len()
    } }

    unsafe fn encode_prefixed(&self, buf : &mut EncodeBuf) { unsafe { match (self) {
        Self::EncryptRequest (packet) => packet.encode_prefixed(buf),
        Self::Finish         (packet) => packet.encode_prefixed(buf),
        Self::Compression    (packet) => packet.encode_prefixed(buf)
    } } }

}

impl<'l> From<S2CLoginPackets<'l>> for S2CPackets<'l> {
    #[inline(always)]
    fn from(value : S2CLoginPackets<'l>) -> Self { Self::Login(value) }
}
