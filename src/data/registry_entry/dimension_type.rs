use super::RegistryEntryType;
use crate::data::{
    ident::Ident,
    light_level::{ LightLevel, LightLevelProvider },
    nbt::to_network as to_network_nbt,
    num::multiple16::{ Multiple16I32, Multiple16U32 },
    tag_ident::TagIdent
};
use std::io::Write;
use serde::Serialize as Ser;


#[derive(Ser, Debug)]
pub struct DimensionTypeRegistryEntry<'l> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_time                      : Option<u64>,
    pub has_skylight                    : bool,
    pub has_ceiling                     : bool,
    #[serde(rename = "ultrawarm")]
    pub is_ultrawarm                    : bool,
    #[serde(rename = "natural")]
    pub is_natural                      : bool,
    #[serde(rename = "coordinate_scale")]
    pub coord_scale                     : f64,
    #[serde(rename = "bed_works")]
    pub beds_work                       : bool,
    #[serde(rename = "respawn_anchor_works")]
    pub anchors_work                    : bool,
    pub min_y                           : Multiple16I32,
    pub height                          : Multiple16U32,
    pub logical_height                  : Multiple16U32,
    #[serde(rename = "infiniburn")]
    pub infiniburn_tag                  : TagIdent,
    pub effects                         : DimensionTypeEffects,
    pub ambient_light                   : f32,
    #[serde(rename = "piglin_safe")]
    pub is_piglin_safe                  : bool,
    pub has_raids                       : bool,
    pub monster_spawn_light_level       : LightLevelProvider<'l>,
    pub monster_spawn_block_light_limit : LightLevel
}

#[derive(Ser, Debug)]
pub enum DimensionTypeEffects {
    #[serde(rename = "minecraft:overworld")]
    Overworld,
    #[serde(rename = "minecraft:the_nether")]
    Nether,
    #[serde(rename = "minecraft:the_end")]
    End
}


impl DimensionTypeRegistryEntry<'_> {
    pub const OVERWORLD : Self = Self {
        fixed_time                      : None,
        has_skylight                    : true,
        has_ceiling                     : false,
        is_ultrawarm                    : false,
        is_natural                      : true,
        coord_scale                     : 1.0,
        beds_work                       : true,
        anchors_work                    : false,
        min_y                           : unsafe { Multiple16I32::new_unchecked(-64) },
        height                          : unsafe { Multiple16U32::new_unchecked(384) },
        logical_height                  : unsafe { Multiple16U32::new_unchecked(384) },
        infiniburn_tag                  : TagIdent::new("#minecraft:infiniburn_overworld"),
        effects                         : DimensionTypeEffects::Overworld,
        ambient_light                   : 0.0,
        is_piglin_safe                  : false,
        has_raids                       : true,
        monster_spawn_light_level       : LightLevelProvider::Uniform {
            min_inclusive : unsafe { LightLevel::new_unchecked(0) },
            max_inclusive : unsafe { LightLevel::new_unchecked(7) }
        },
        monster_spawn_block_light_limit : unsafe { LightLevel::new_unchecked(0) }
    };
}

impl Default for DimensionTypeRegistryEntry<'_> {
    #[inline(always)]
    fn default() -> Self { Self::OVERWORLD }
}


impl RegistryEntryType for DimensionTypeRegistryEntry<'_> {
    const REGISTRY_ID : Ident = Ident::new("minecraft:dimension_type");

    fn to_network_nbt<W>(&self, writer : W) -> bool
    where
        W : Write
    {
        to_network_nbt(writer, self).unwrap();
        true
    }
}
