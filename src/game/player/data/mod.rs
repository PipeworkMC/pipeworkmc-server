use pipeworkmc_data::{
    client_info::ClientInfo,
    game_mode::GameMode
};
use core::num::NonZeroU8;
use bevy_ecs::{
    bundle::Bundle,
    component::Component
};


pub mod dimension;
use dimension::Dimension;


#[derive(Bundle, Default)]
pub struct PlayerBundle {
    client_info : ClientInfo,
    dimension   : Dimension,
    view_dist   : ViewDist,
    game_mode   : GameMode
}


#[derive(Component)]
pub struct ClientBrand {
    pub brand : String
}

#[derive(Component)]
pub struct IsHardcore;

#[derive(Component)]
pub struct ReducedDebugInfo; // TODO: Detect changes and update player RDI.

#[derive(Component)]
pub struct NoRespawnScreen; // TODO: Detect changes and update player respawn screen.


#[derive(Component)]
pub struct ViewDist(NonZeroU8);
impl ViewDist {
    #[inline(always)]
    pub fn as_n0u8(&self) -> NonZeroU8 { self.0 }
    #[inline(always)]
    pub fn as_u8(&self) -> u8 { self.0.get() }
}
impl Default for ViewDist {
    fn default() -> Self { Self(unsafe { NonZeroU8::new_unchecked(8) }) }
}
