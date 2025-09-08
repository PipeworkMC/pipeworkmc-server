use crate::conn::protocol::{
    codec::{
        encode::{
            PacketEncode,
            EncodeBuf,
            slice::UnprefixedVec
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
use crate::data::{
    ident::Ident,
    registry_entry::RegistryEntryType
};
use core::fmt::Debug;


#[derive(Debug)]
pub struct S2CConfigRegistryDataPacket {
    registry : Ident,
    entries  : Vec<(Ident, Option<UnprefixedVec<u8>>,)>
}


impl PacketMeta for S2CConfigRegistryDataPacket {
    const STATE  : PacketState = PacketState::Config;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x07; // TODO: Check against current datagen.
}

unsafe impl PacketEncode for S2CConfigRegistryDataPacket {

    #[inline(always)]
    fn encode_len(&self) -> usize {
        self.registry.encode_len()
        + self.entries.encode_len()
    }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.registry.encode(buf);
        self.entries.encode(buf);
    } }

}

impl From<S2CConfigRegistryDataPacket> for S2CPackets<'_> {
    #[inline(always)]
    fn from(value : S2CConfigRegistryDataPacket) -> Self { Self::Config(value.into()) }
}

impl From<S2CConfigRegistryDataPacket> for S2CConfigPackets<'_> {
    #[inline(always)]
    fn from(value : S2CConfigRegistryDataPacket) -> Self { Self::RegistryData(value) }
}


impl<I, T> From<I> for S2CConfigRegistryDataPacket
where
    I : IntoIterator<Item = (Ident, T,)>,
    T : RegistryEntryType
{
    fn from(entries : I) -> Self { Self {
        registry : T::REGISTRY_ID,
        entries  : entries.into_iter().map(|(entry_id, entry,)| {
            let mut data    = Vec::new();
            let     is_some = entry.to_network_nbt(&mut data);
            (entry_id, is_some.then(|| UnprefixedVec::from(data)),)
        }).collect::<Vec<_>>()
    } }
}
