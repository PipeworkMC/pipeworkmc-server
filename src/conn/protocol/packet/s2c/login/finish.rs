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
pub struct S2CLoginFinishPacket<'l> {
    pub profile : Profile<'l>
}


impl PacketMeta for S2CLoginFinishPacket<'_> {
    const STATE  : PacketState = PacketState::Login;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x02; // TODO: Check against current datagen.
}

unsafe impl PacketEncode for S2CLoginFinishPacket<'_> {

    #[inline(always)]
    fn encode_len(&self) -> usize {
        self.profile.encode_len()
    }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.profile.encode(buf);
    } }

}

impl<'l> From<S2CLoginFinishPacket<'l>> for S2CPackets<'l> {
    #[inline(always)]
    fn from(value : S2CLoginFinishPacket<'l>) -> Self { Self::Login(value.into()) }
}

impl<'l> From<S2CLoginFinishPacket<'l>> for S2CLoginPackets<'l> {
    #[inline(always)]
    fn from(value : S2CLoginFinishPacket<'l>) -> Self { Self::Finish(value) }
}
