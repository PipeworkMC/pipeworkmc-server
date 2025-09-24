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


#[derive(Bundle, Default)]
pub struct PlayerCharacterBundle {
    pub player      : PlayerCharacter,
    pub client_info : ClientInfo,
    pub dimension   : Dimension,
    pub view_dist   : ViewDist,
    pub game_mode   : GameMode,
    pub character   : super::CharacterBundle
}


#[derive(Component, Default)]
pub struct PlayerCharacter;

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
