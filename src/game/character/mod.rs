use pipeworkmc_data::{
    character::{
        CharacterId,
        CharacterPos,
        CharacterRot,
        CharacterVel,
        CharacterType
    },
    uuid::Uuid
};
use bevy_app::{
    App, Plugin,
    PreUpdate, Update
};
use bevy_ecs::{
    component::Component,
    bundle::Bundle
};


pub(crate) mod vis;
pub use vis::CharacterVisibility;

pub mod player;


#[derive(Bundle, Default)]
pub struct CharacterBundle {
    pub eid        : CharacterId,
    pub pos        : CharacterPos,
    pub rot        : CharacterRot,
    pub vel        : CharacterVel,
    pub visibility : CharacterVisibility
}

#[derive(Component)]
#[component(immutable)]
pub struct Character {
    ty   : CharacterType,
    uuid : Uuid,
    data : u32
}
impl Character {
    #[inline(always)]
    pub fn ty(&self) -> CharacterType { self.ty }
    #[inline(always)]
    pub fn uuid(&self) -> Uuid { self.uuid }
    #[inline(always)]
    pub fn data(&self) -> u32 { self.data }
}


pub struct CharactersPlugin;
impl Plugin for CharactersPlugin {
    fn build(&self, app : &mut App) {
        app
            .add_systems(Update, vis::update_visibilities)
            .add_systems(Update, vis::on_remove_character)

            .add_systems(PreUpdate, player::set_character)
        ;
    }
}
