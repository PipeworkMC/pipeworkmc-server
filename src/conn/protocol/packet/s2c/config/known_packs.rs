use crate::conn::protocol::{
    codec::{
        encode::{
            PacketEncode,
            EncodeBuf
        },
        meta::{
            PacketMeta,
            PacketState,
            PacketBound
        }
    },
    packet::s2c::{
        S2CPackets,
        config::S2CConfigPackets
    }
};
use crate::data::known_pack::KnownPack;
use std::borrow::Cow;


#[derive(Debug)]
pub struct S2CConfigKnownPacksPacket<'l> {
    pub known_packs : Cow<'l, [KnownPack<'l>]>
}


impl PacketMeta for S2CConfigKnownPacksPacket<'_> {
    const STATE  : PacketState = PacketState::Config;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x0E; // TODO: Check against current datagen.
}

unsafe impl PacketEncode for S2CConfigKnownPacksPacket<'_> {

    #[inline(always)]
    fn encode_len(&self) -> usize {
        self.known_packs.encode_len()
    }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.known_packs.encode(buf);
    } }

}

impl<'l> From<S2CConfigKnownPacksPacket<'l>> for S2CPackets<'l> {
    #[inline(always)]
    fn from(value : S2CConfigKnownPacksPacket<'l>) -> Self { Self::Config(value.into()) }
}

impl<'l> From<S2CConfigKnownPacksPacket<'l>> for S2CConfigPackets<'l> {
    #[inline(always)]
    fn from(value : S2CConfigKnownPacksPacket<'l>) -> Self { Self::KnownPacks(value) }
}
