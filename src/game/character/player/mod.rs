//! Player data and systems.


use super::Character;
use pipeworkmc_data::{
    character::CharacterType,
    client_info::ClientInfo,
    game_mode::GameMode,
    profile::AccountProfile
};
use core::num::NonZeroU8;
use bevy_ecs::{
    bundle::Bundle,
    component::Component,
    entity::Entity,
    query::Added,
    system::{ Commands, Query }
};


mod dimension;
pub use dimension::*;


/// A [`Bundle`] of [`Component`]s required to create a player character.
#[derive(Bundle, Default)]
pub struct PlayerCharacterBundle {
    /// A marker for player-type characters.
    pub player      : PlayerCharacter,
    /// Client settings.
    pub client_info : ClientInfo,
    /// The dimension that the player is in.
    pub dimension   : Dimension,
    /// The player's view distance.
    pub view_dist   : ViewDist,
    /// The player's game mode.
    pub game_mode   : GameMode,
    /// General character components.
    pub character   : super::CharacterBundle
}


/// A marker for player-type characters.
#[derive(Component, Default)]
#[component(immutable)]
#[non_exhaustive]
pub struct PlayerCharacter;

/// Iterates through all characters marked as player-type, and sets data used by entity spawners.
pub(in crate::game::character) fn set_character(
    mut cmds        : Commands,
        q_character : Query<(Entity, &AccountProfile), (Added<PlayerCharacter>,)>
) {
    for (entity, profile,) in &q_character {
        cmds.entity(entity).insert(Character {
            ty   : CharacterType::Player,
            uuid : profile.uuid,
            data : 0
        });
    }
}


/// The client's declared brand.
#[derive(Component)]
pub struct ClientBrand {
    /// The brand (Vanilla, fabric, forge, etc).
    pub brand : String
}

/// Whether the player should be logged in with hardcore hearts.
///
/// Can not be changed after logging in.
#[derive(Component)]
#[component(immutable)]
pub struct IsHardcore;

/// Whether the player's F3 debug screen is reduced.
#[derive(Component)]
pub struct ReducedDebugInfo; // TODO: Detect changes and update player RDI.

/// Whether the player will immediately respawn without showing the respawn screen on death.
#[derive(Component)]
pub struct NoRespawnScreen; // TODO: Detect changes and update player respawn screen.


/// The player's view distance.
#[derive(Component)]
pub struct ViewDist(NonZeroU8);
impl ViewDist {
    /// Returns the inner value as a [`NonZeroU8`].
    #[inline(always)]
    pub fn as_n0u8(&self) -> NonZeroU8 { self.0 }
    /// Returns the inner value as a [`u8`].
    #[inline(always)]
    pub fn as_u8(&self) -> u8 { self.0.get() }
}
impl Default for ViewDist {
    #[inline(always)]
    fn default() -> Self {
        // SAFETY: 8 is not 0.
        Self(unsafe { NonZeroU8::new_unchecked(8) })
    }
}
