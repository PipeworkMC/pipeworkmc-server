use core::num::NonZeroU8;
use bevy_ecs::component::Component;


pub mod dimension;


#[derive(Component)]
pub struct IsHardcore;

#[derive(Component)]
pub struct ReducedDebugInfo; // TODO: Detect changes and update player RDI.

#[derive(Component)]
pub struct NoRespawnScreen; // TODO: Detect changes and update player respawn screen.


#[derive(Component)]
pub struct ViewDistance(NonZeroU8);
impl ViewDistance {
    #[inline(always)]
    pub fn as_n0u8(&self) -> NonZeroU8 { self.0 }
    #[inline(always)]
    pub fn as_u8(&self) -> u8 { self.0.get() }
}
impl Default for ViewDistance {
    fn default() -> Self { Self(unsafe { NonZeroU8::new_unchecked(8) }) }
}
