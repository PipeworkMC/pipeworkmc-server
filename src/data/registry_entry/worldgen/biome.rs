use super::RegistryEntryType;
use crate::data::{
    colour::Rgb,
    ident::Ident,
    nbt::to_network as to_network_nbt,
    particle::Particle
};
use crate::util::is_default;
use std::io::Write;
use serde::Serialize as Ser;


#[derive(Ser, Debug)]
pub struct WorldgenBiomeRegistryEntry {
    #[serde(rename = "has_precipitation")]
    pub can_rain             : bool,
    pub temperature          : f32,
    #[serde(skip_serializing_if = "is_default")]
    pub temperature_modifier : WorldgenBiomeTemperatureModifier,
    #[serde(rename = "downfall")]
    pub downfall_factor      : f32,
    pub effects              : WorldgenBiomeEffects
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Ser, Debug)]
pub enum WorldgenBiomeTemperatureModifier {
    #[default]
    #[serde(rename = "none")]
    None,
    #[serde(rename = "frozen")]
    Frozen
}

#[derive(Ser, Debug)]
pub struct WorldgenBiomeEffects {
    #[serde(rename = "fog_color")]
    pub fog_colour            : Rgb,
    #[serde(rename = "water_color")]
    pub water_color           : Rgb,
    #[serde(rename = "water_fog_color")]
    pub water_fog_colour      : Rgb,
    #[serde(rename = "sky_color")]
    pub sky_colour            : Rgb,
    #[serde(rename = "foliage_color", skip_serializing_if = "Option::is_none")]
    pub foliage_colour        : Option<Rgb>,
    #[serde(rename = "grass_color", skip_serializing_if = "Option::is_none")]
    pub grass_colour          : Option<Rgb>,
    #[serde(rename = "grass_color_modifier", skip_serializing_if = "is_default")]
    pub grass_colour_modifier : WorldgenBiomeGrassColourModifier,
    pub particle              : WorldgenBiomeParticle,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ambient_sound         : Option<WorldgenBiomeAmbientSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mood_sound            : Option<WorldgenBiomeMoodSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additions_sound       : Option<WorldgenBiomeAdditionsSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music                 : Option<WorldgenBiomeMusic>
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Ser, Debug)]
pub enum WorldgenBiomeGrassColourModifier {
    #[default]
    #[serde(rename = "none")]
    None,
    #[serde(rename = "dark_forest")]
    DarkForest,
    #[serde(rename = "swamp")]
    Swamp
}

#[derive(Ser, Debug)]
pub struct WorldgenBiomeParticle {
    pub options     : Particle,
    pub probability : f32
}

#[derive(Ser, Debug)]
pub struct WorldgenBiomeAmbientSound {
    #[serde(rename = "sound_id")]
    pub sound : Ident,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range  : Option<f32>
}

#[derive(Ser, Debug)]
pub struct WorldgenBiomeMoodSound {
    pub sound               : Ident,
    pub tick_delay          : u32,
    pub block_search_extent : u32,
    pub offset              : f64
}

#[derive(Ser, Debug)]
pub struct WorldgenBiomeAdditionsSound {
    pub sound       : Ident,
    pub tick_chance : f64
}

#[derive(Ser, Debug)]
pub struct WorldgenBiomeMusic {
    pub sound           : Ident,
    pub min_delay       : u32,
    pub max_delay       : u32,
    #[serde(rename = "replace_current_music")]
    pub replace_current : bool
}


impl RegistryEntryType for WorldgenBiomeRegistryEntry {
    const REGISTRY_ID : Ident = Ident::new("minecraft:worldgen/biome");

    fn to_network_nbt<W>(&self, writer : W) -> bool
    where
        W : Write
    {
        to_network_nbt(writer, self).unwrap();
        true
    }
}
