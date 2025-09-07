use super::RegistryEntryType;
use crate::data::{
    ident::Ident,
    nbt::to_network as to_network_nbt
};
use crate::util::is_default;
use std::{
    borrow::Cow,
    io::Write
};
use serde::Serialize as Ser;


#[derive(Ser, Debug)]
pub struct DamageTypeRegistryEntry<'l> {
    pub message_id    : Cow<'l, str>,
    pub scaling       : DamageTypeScaling,
    pub exhaustion    : f32,
    #[serde(skip_serializing_if = "is_default")]
    pub effects       : DamageTypeEffects,
    #[serde(skip_serializing_if = "is_default")]
    pub death_message : DamageTypeDeathMessage
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Ser, Debug)]
pub enum DamageTypeScaling {
    #[serde(rename = "never")]
    Never,
    #[serde(rename = "when_caused_by_living_non_player")]
    WhenByEnemy,
    #[serde(rename = "always")]
    Always
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Ser, Debug)]
pub enum DamageTypeEffects {
    #[default]
    #[serde(rename = "hurt")]
    Hurt,
    #[serde(rename = "thorns")]
    Thorns,
    #[serde(rename = "drowning")]
    Drowning,
    #[serde(rename = "burning")]
    Burning,
    #[serde(rename = "poking")]
    Poking,
    #[serde(rename = "freezing")]
    Freezing
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Ser, Debug)]
pub enum DamageTypeDeathMessage {
    #[default]
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "fall_variants")]
    FallVariants,
    #[serde(rename = "intentional_game_design")]
    IntentionalGameDesign
}


impl RegistryEntryType for DamageTypeRegistryEntry<'_> {
    const REGISTRY_ID : Ident = Ident::new("minecraft:damage_type");

    fn to_network_nbt<W>(&self, writer : W) -> bool
    where
        W : Write
    {
        to_network_nbt(writer, self).unwrap();
        true
    }
}
