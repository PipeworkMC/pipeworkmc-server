use super::RegistryEntryType;
use crate::data::{
    ident::Ident,
    nbt::to_network as to_network_nbt,
    text::Text
};
use std::io::Write;
use serde::Serialize as Ser;


#[derive(Ser, Debug)]
pub struct TrimPatternRegistryEntry {
    #[serde(rename = "asset_id")]
    pub pattern_asset : Ident,
    #[serde(rename = "template_item")]
    pub template      : Ident, // TODO: Replace with item type enum.
    #[serde(rename = "description")]
    pub tooltip       : Text,
    #[serde(rename = "decal")]
    pub is_decal      : bool
}


impl RegistryEntryType for TrimPatternRegistryEntry {
    const REGISTRY_ID : Ident = Ident::new("minecraft:trim_pattern");

    fn to_network_nbt<W>(&self, writer : W) -> bool
    where
        W : Write
    {
        to_network_nbt(writer, self).unwrap();
        true
    }
}
