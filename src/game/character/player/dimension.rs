use pipeworkmc_data::{
    ident::Ident,
    dimension_type::DimensionType
};
use bevy_ecs::component::Component;


/// The dimension that the player is in.
#[derive(Component)]
pub struct Dimension { // TODO: Detect changes and reload player dimension.
    /// ID of the dimension.
    pub id          : Ident,
    /// Hashed seed, used for noise in grass colour.
    pub hashed_seed : u64,
    /// Whether this is a debug world.
    pub is_debug    : bool,
    /// Whether this is a superflat world.
    pub is_flat     : bool,
    /// The sea level of this dimension.
    pub sea_level   : i32,
    /// Properties of this dimension.
    pub dim_type    : DimensionType<'static>
}

impl Dimension {
    /// The vanilla overworld dimension.
    pub const OVERWORLD : Self = Self {
        id          : Ident::new("minecraft:overworld"),
        hashed_seed : 0,
        is_debug    : false,
        is_flat     : true,
        sea_level   : 64,
        dim_type    : DimensionType::OVERWORLD
    };
}

impl Default for Dimension {
    #[inline]
    fn default() -> Self { Self::OVERWORLD }
}
