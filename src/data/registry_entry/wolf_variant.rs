use super::RegistryEntryType;
use crate::data::{
    ident::Ident,
    nbt::to_network as to_network_nbt
};
use std::{
    borrow::Cow,
    io::Write
};
use serde::Serialize as Ser;


#[derive(Ser, Debug)]
pub struct WolfVariantRegistryEntry<'l> {
    pub assets : WolfVariantAssets,
    pub biomes : Cow<'l, [Ident]>
}

#[derive(Ser, Debug)]
pub struct WolfVariantAssets {
    pub wild  : Ident,
    pub tame  : Ident,
    pub angry : Ident
}


impl RegistryEntryType for WolfVariantRegistryEntry<'_> {
    const REGISTRY_ID : Ident = Ident::new("minecraft:wolf_variant");

    fn to_network_nbt<W>(&self, writer : W) -> bool
    where
        W : Write
    {
        to_network_nbt(writer, self).unwrap();
        true
    }
}
