use crate::conn::protocol::codec::encode::{
    PrefixedPacketEncode,
    EncodeBuf
};


pub mod status;


#[derive(Debug)]
pub enum S2CPackets {
    Status(status::S2CStatusPackets),
    // TODO: Login
    // TODO: Config
    // TODO: Play
}

unsafe impl PrefixedPacketEncode for S2CPackets {

    fn encode_prefixed_len(&self) -> usize { match (self) {
        S2CPackets::Status (packet) => packet.encode_prefixed_len()
    } }

    unsafe fn encode_prefixed(&self, buf : &mut EncodeBuf) { unsafe { match (self) {
        S2CPackets::Status (packet) => packet.encode_prefixed(buf)
    } } }

}
