//! Character data and systems.


use pipeworkmc_data::{
    character::{
        CharacterPos,
        CharacterRot,
        CharacterVel,
        CharacterType
    },
    uuid::Uuid
};
use bevy_app::{
    App, Plugin,
    PostUpdate
};
use bevy_ecs::{
    component::Component,
    bundle::Bundle
};


pub(crate) mod vis;
pub use vis::CharacterVisibility;

pub mod player;


/// A [`Bundle`] of [`Component`]s required to create any character.
///
/// It is not recommended that this is used by itself.
#[derive(Bundle, Default)]
pub struct CharacterBundle {
    /// The position of this character.
    pub pos        : CharacterPos,
    /// The rotation of this character.
    pub rot        : CharacterRot,
    /// The velocity of this character.
    pub vel        : CharacterVel,
    /// What players this character is visible to.
    pub visibility : CharacterVisibility
}

/// A marker for all characters.
///
/// Can not be changed after spawning.
#[derive(Component)]
#[component(immutable)]
#[non_exhaustive]
pub struct Character {
    /// The type of character.
    pub ty   : CharacterType,
    /// The UUID of this character.
    pub uuid : Uuid,
    /// Special data used when spawning this character.
    ///
    /// [https://minecraft.wiki/w/Java_Edition_protocol/Object_data]
    pub data : u32
}


/// A [`Plugin`] which manages spawning and tracking of characters.
pub struct CharactersPlugin;
impl Plugin for CharactersPlugin {
    fn build(&self, app : &mut App) {
        app
            .add_systems(PostUpdate, vis::update_visibilities)
            .add_systems(PostUpdate, vis::on_remove_character)

            // Player
            .add_observer(player::set_character)
            .add_systems(PostUpdate, player::update_client_info)
            .add_systems(PostUpdate, player::update_game_mode)
            .add_systems(PostUpdate, player::update_no_respawn_screen)
        ;
    }
}
