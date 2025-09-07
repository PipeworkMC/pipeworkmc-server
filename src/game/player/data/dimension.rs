use crate::data::{
    ident::Ident,
    registry_entry::dimension_type::DimensionTypeRegistryEntry
};
use bevy_ecs::component::Component;


#[derive(Component)]
pub struct Dimension { // TODO: Detect changes and reload player dimension.
    pub id          : Ident,
    pub hashed_seed : u64,
    pub is_debug    : bool,
    pub is_flat     : bool,
    pub sea_level   : i32,
    pub dim_type    : DimensionTypeRegistryEntry<'static>
}

impl Dimension {
    pub const OVERWORLD : Self = Self {
        id          : Ident::new("minecraft:overworld"),
        hashed_seed : 0,
        is_debug    : false,
        is_flat     : true,
        sea_level   : 64,
        dim_type    : DimensionTypeRegistryEntry::OVERWORLD
    };
}

impl Default for Dimension {
    #[inline(always)]
    fn default() -> Self { Self::OVERWORLD }
}
