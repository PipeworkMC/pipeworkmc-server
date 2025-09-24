use pipeworkmc_data::{
    bounded_string::BoundedString,
    uuid::Uuid
};
use bevy_ecs::{
    entity::Entity,
    event::Event
};


#[derive(Event, Debug)]
pub struct PlayerLoggedOutEvent { // TODO: Fire logout event.
    peer     : Entity,
    uuid     : Uuid,
    username : BoundedString<16>
}

impl PlayerLoggedOutEvent {
    #[inline(always)]
    pub(crate) fn new(peer : Entity, uuid : Uuid, username : BoundedString<16>) -> Self { Self {
        peer, uuid, username
    } }
}

impl PlayerLoggedOutEvent {

    #[inline(always)]
    pub fn peer(&self) -> Entity { self.peer }

    #[inline(always)]
    pub fn uuid(&self) -> Uuid { self.uuid }
    #[inline(always)]
    pub fn username(&self) -> &BoundedString<16> { &self.username }

}
