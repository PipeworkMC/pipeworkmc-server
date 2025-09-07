use super::RegistryEntryType;
use crate::data::{
    ident::Ident,
    nbt::to_network as to_network_nbt,
    text::Text
};
use core::num::NonZeroU32;
use std::io::Write;
use serde::Serialize as Ser;


#[derive(Ser, Debug)]
pub struct PaintingVariantRegistryEntry {
    #[serde(rename = "asset_id")]
    pub texture_asset : Ident,
    pub height        : NonZeroU32,
    pub width         : NonZeroU32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title         : Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author        : Option<Text>
}


impl RegistryEntryType for PaintingVariantRegistryEntry {
    const REGISTRY_ID : Ident = Ident::new("minecraft:painting_variant");

    fn to_network_nbt<W>(&self, writer : W) -> bool
    where
        W : Write
    {
        to_network_nbt(writer, self).unwrap();
        true
    }
}
