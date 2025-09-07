use super::RegistryEntryType;
use crate::data::{
    ident::Ident,
    nbt::to_network as to_network_nbt
};
use std::io::Write;
use serde::Serialize as Ser;


#[derive(Ser, Debug)]
pub struct WolfSoundVariantRegistryEntry {
    pub hurt_sound    : Ident,
    pub pant_sound    : Ident,
    pub whine_sound   : Ident,
    pub ambient_sound : Ident,
    pub death_sound   : Ident,
    pub growl_sound   : Ident
}


impl RegistryEntryType for WolfSoundVariantRegistryEntry {
    const REGISTRY_ID : Ident = Ident::new("minecraft:wolf_sound_variant");

    fn to_network_nbt<W>(&self, writer : W) -> bool
    where
        W : Write
    {
        to_network_nbt(writer, self).unwrap();
        true
    }
}
