use super::RegistryEntryType;
use crate::data::{
    ident::Ident,
    nbt::to_network as to_network_nbt,
    text::Text
};
use std::io::Write;
use serde::Serialize as Ser;


#[derive(Ser, Debug)]
pub struct TrimMaterialRegistryEntry {
    #[serde(rename = "asset_name")]
    pub palette_asset     : Ident,
    pub ingredient        : Ident, // TODO: Replace with item type enum.
    #[serde(rename = "item_model_index")]
    pub item_model_index  : f32,
    #[serde(rename = "override_armor_materials")]
    pub palette_overrides : TrimMaterialPaletteOverrides,
    #[serde(rename = "description")]
    pub tooltip           : Text
}

#[derive(Ser, Debug, Default)]
pub struct TrimMaterialPaletteOverrides {
    #[serde(rename = "leather", skip_serializing_if = "Option::is_none")]
    pub leather_asset   : Option<Ident>,
    #[serde(rename = "chainmail", skip_serializing_if = "Option::is_none")]
    pub chainmail_asset : Option<Ident>,
    #[serde(rename = "iron", skip_serializing_if = "Option::is_none")]
    pub iron_asset      : Option<Ident>,
    #[serde(rename = "gold", skip_serializing_if = "Option::is_none")]
    pub gold_asset      : Option<Ident>,
    #[serde(rename = "diamond", skip_serializing_if = "Option::is_none")]
    pub diamond_asset   : Option<Ident>,
    #[serde(rename = "turtle", skip_serializing_if = "Option::is_none")]
    pub turtle_asset    : Option<Ident>,
    #[serde(rename = "netherite", skip_serializing_if = "Option::is_none")]
    pub netherite_asset : Option<Ident>
}


impl RegistryEntryType for TrimMaterialRegistryEntry {
    const REGISTRY_ID : Ident = Ident::new("minecraft:trim_material");

    fn to_network_nbt<W>(&self, writer : W) -> bool
    where
        W : Write
    {
        to_network_nbt(writer, self).unwrap();
        true
    }
}
