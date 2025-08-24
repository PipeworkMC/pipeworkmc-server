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
            login::S2CLoginPackets
        }
    },
    value::bounded_string::BoundedString
};
use crate::util::redacted::Redacted;
use std::borrow::Cow;


#[derive(Debug)]
pub struct S2CLoginEncryptRequestPacket {
    pub server_id       : BoundedString<20>,
    pub public_key      : Redacted<Cow<'static, [u8]>>,
    pub verify_token    : [u8; 4],
    pub mojauth_enabled : bool
}

impl PacketMeta for S2CLoginEncryptRequestPacket {
    const STATE  : PacketState = PacketState::Login;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x01; // TODO: Check against current datagen.
}

unsafe impl PacketEncode for S2CLoginEncryptRequestPacket {

    fn encode_len(&self) -> usize {
        self.server_id.encode_len()
        + unsafe { self.public_key.as_ref() }.encode_len()
        + self.verify_token.encode_len()
        + self.mojauth_enabled.encode_len()
    }

    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.server_id.encode(buf);
        self.public_key.as_ref().encode(buf);
        self.verify_token.as_slice().encode(buf);
        self.mojauth_enabled.encode(buf);
    } }

}

impl From<S2CLoginEncryptRequestPacket> for S2CPackets {
    #[inline(always)]
    fn from(value : S2CLoginEncryptRequestPacket) -> Self { Self::Login(value.into()) }
}

impl From<S2CLoginEncryptRequestPacket> for S2CLoginPackets {
    #[inline(always)]
    fn from(value : S2CLoginEncryptRequestPacket) -> Self { Self::EncryptRequest(value) }
}
