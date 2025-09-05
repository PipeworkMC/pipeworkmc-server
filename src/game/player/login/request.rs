use crate::data::bounded_string::BoundedString;
use bevy_ecs::{
    entity::Entity,
    event::Event
};
use uuid::Uuid;


#[derive(Event, Debug)]
pub struct PlayerRequestLoginEvent {
    peer     : Entity,
    uuid     : Uuid,
    username : BoundedString<16>
}

impl PlayerRequestLoginEvent {
    #[inline(always)]
    pub(crate) fn new(peer : Entity, uuid : Uuid, username : BoundedString<16>) -> Self { Self {
        peer, uuid, username
    } }
}

impl PlayerRequestLoginEvent {

    #[inline(always)]
    pub fn peer(&self) -> Entity { self.peer }

    #[inline(always)]
    pub fn uuid(&self) -> Uuid { self.uuid }
    #[inline(always)]
    pub fn username(&self) -> &BoundedString<16> { &self.username }

}
