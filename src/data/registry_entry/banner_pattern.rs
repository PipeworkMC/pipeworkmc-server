use super::RegistryEntryType;
use crate::data::{
    ident::Ident,
    nbt::to_network as to_network_nbt,
    text::Text
};
use std::io::Write;
use serde::Serialize as Ser;


#[derive(Ser, Debug)]
pub struct BannerPatternRegistryEntry {
    #[serde(rename = "asset_id")]
    pub pattern_asset : Ident,
    #[serde(rename = "translation_key")]
    pub tooltip       : Text
}


impl RegistryEntryType for BannerPatternRegistryEntry {
    const REGISTRY_ID : Ident = Ident::new("minecraft:banner_pattern");

    fn to_network_nbt<W>(&self, writer : W) -> bool
    where
        W : Write
    {
        to_network_nbt(writer, self).unwrap();
        true
    }
}
