use pipeworkmc_data::{
    bounded_string::BoundedString,
    uuid::Uuid
};
use bevy_ecs::{
    entity::Entity,
    event::Event
};


/// An [`Event`] which is emitted when a player successfully logs in.
#[derive(Event, Debug)]
#[non_exhaustive]
pub struct PlayerLoggedInEvent {
    /// The [`Entity`] of the peer who logged in.
    pub peer     : Entity,
    /// The UUID of the player who logged in.
    pub uuid     : Uuid,
    /// The username of the player who logged in.
    pub username : BoundedString<16>
}

impl PlayerLoggedInEvent {
    #[inline(always)]
    pub(crate) fn new(peer : Entity, uuid : Uuid, username : BoundedString<16>) -> Self {
        Self { peer, uuid, username }
    }
}
