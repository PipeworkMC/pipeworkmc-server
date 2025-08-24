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
    value::profile::Profile
};


#[derive(Debug)]
pub struct S2CLoginSuccessPacket {
    pub profile : Profile
}


impl PacketMeta for S2CLoginSuccessPacket {
    const STATE  : PacketState = PacketState::Login;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x02; // TODO: Check against current datagen.
}

unsafe impl PacketEncode for S2CLoginSuccessPacket {

    #[inline(always)]
    fn encode_len(&self) -> usize {
        self.profile.encode_len()
    }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.profile.encode(buf);
    } }

}

impl From<S2CLoginSuccessPacket> for S2CPackets {
    #[inline(always)]
    fn from(value : S2CLoginSuccessPacket) -> Self { Self::Login(value.into()) }
}

impl From<S2CLoginSuccessPacket> for S2CLoginPackets {
    #[inline(always)]
    fn from(value : S2CLoginSuccessPacket) -> Self { Self::Success(value) }
}
