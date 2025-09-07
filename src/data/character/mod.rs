use bevy_ecs::{
    component::Component,
    resource::Resource
};


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Component)]
pub struct CharacterId(u32);

impl CharacterId {

    pub const ZERO : Self = Self(0);

    pub const fn new(id : u32) -> Self { Self(id) }

    pub const fn as_u32(&self) -> u32 { self.0 }

}

impl From<u32> for CharacterId {
    fn from(id : u32) -> Self { Self::new(id) }
}


#[derive(Resource, Default)]
pub struct NextCharacterId(u32);

impl NextCharacterId {

    pub fn next(&mut self) -> CharacterId {
        let id = self.0;
        self.0 += 1;
        CharacterId(id)
    }

}
