use crate::conn::protocol::{
    codec::encode::{
        PrefixedPacketEncode,
        EncodeBuf
    },
    packet::PacketState
};


pub mod status;
pub mod login;
pub mod config;
pub mod play;


#[derive(Debug)]
pub enum S2CPackets<'l> {
    Status(status::S2CStatusPackets<'l>),
    Login(login::S2CLoginPackets<'l>),
    Config(config::S2CConfigPackets<'l>),
    Play(play::S2CPlayPackets)
}

impl S2CPackets<'_> {

    pub fn meta(&self) -> (PacketState, u8,) { match (self) {
        Self::Status (packet) => (PacketState::Status, packet.prefix(),),
        Self::Login  (packet) => (PacketState::Login,  packet.prefix(),),
        Self::Config (packet) => (PacketState::Config, packet.prefix(),),
        Self::Play   (packet) => (PacketState::Play,   packet.prefix(),)
    } }

}

unsafe impl PrefixedPacketEncode for S2CPackets<'_> {

    fn encode_prefixed_len(&self) -> usize { match (self) {
        S2CPackets::Status (packet) => packet.encode_prefixed_len(),
        S2CPackets::Login  (packet) => packet.encode_prefixed_len(),
        S2CPackets::Config (packet) => packet.encode_prefixed_len(),
        S2CPackets::Play   (packet) => packet.encode_prefixed_len()
    } }

    unsafe fn encode_prefixed(&self, buf : &mut EncodeBuf) { unsafe { match (self) {
        S2CPackets::Status (packet) => packet.encode_prefixed(buf),
        S2CPackets::Login  (packet) => packet.encode_prefixed(buf),
        S2CPackets::Config (packet) => packet.encode_prefixed(buf),
        S2CPackets::Play   (packet) => packet.encode_prefixed(buf)
    } } }

}
