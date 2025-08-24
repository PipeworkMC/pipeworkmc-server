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
pub mod encrypt_request;
pub mod finish;
pub mod compression;


#[derive(Debug)]
pub enum S2CLoginPackets<'l> {
    Disconnect     (disconnect      ::S2CLoginDisconnectPacket<'l>),
    EncryptRequest (encrypt_request ::S2CLoginEncryptRequestPacket<'l>),
    Finish         (finish          ::S2CLoginFinishPacket<'l>),
    Compression    (compression     ::S2CLoginCompressionPacket)
    // TODO: QueryRequest
    // TODO: CookieRequest
}

impl S2CLoginPackets<'_> {

    pub fn prefix(&self) -> u8 { match (self) {
        Self::Disconnect     (_) => disconnect      ::S2CLoginDisconnectPacket     ::PREFIX,
        Self::EncryptRequest (_) => encrypt_request ::S2CLoginEncryptRequestPacket ::PREFIX,
        Self::Finish         (_) => finish          ::S2CLoginFinishPacket         ::PREFIX,
        Self::Compression    (_) => compression     ::S2CLoginCompressionPacket    ::PREFIX
    } }

}

unsafe impl PrefixedPacketEncode for S2CLoginPackets<'_> {

    fn encode_prefixed_len(&self) -> usize { match (self) {
        Self::Disconnect     (packet) => packet.encode_prefixed_len(),
        Self::EncryptRequest (packet) => packet.encode_prefixed_len(),
        Self::Finish         (packet) => packet.encode_prefixed_len(),
        Self::Compression    (packet) => packet.encode_prefixed_len()
    } }

    unsafe fn encode_prefixed(&self, buf : &mut EncodeBuf) { unsafe { match (self) {
        Self::Disconnect     (packet) => packet.encode_prefixed(buf),
        Self::EncryptRequest (packet) => packet.encode_prefixed(buf),
        Self::Finish         (packet) => packet.encode_prefixed(buf),
        Self::Compression    (packet) => packet.encode_prefixed(buf)
    } } }

}

impl<'l> From<S2CLoginPackets<'l>> for S2CPackets<'l> {
    #[inline(always)]
    fn from(value : S2CLoginPackets<'l>) -> Self { Self::Login(value) }
}
