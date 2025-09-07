use super::RegistryEntryType;
use crate::data::{
    ident::Ident,
    nbt::to_network as to_network_nbt
};
use std::io::Write;
use serde::Serialize as Ser;


#[derive(Ser, Debug)]
pub struct CowVariantRegistryEntry {
    #[serde(rename = "asset_id")]
    pub texture_asset : Ident
}


impl RegistryEntryType for CowVariantRegistryEntry {
    const REGISTRY_ID : Ident = Ident::new("minecraft:cow_variant");

    fn to_network_nbt<W>(&self, writer : W) -> bool
    where
        W : Write
    {
        to_network_nbt(writer, self).unwrap();
        true
    }
}
