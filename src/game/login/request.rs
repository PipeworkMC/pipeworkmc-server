use pipeworkmc_data::{
    bounded_string::BoundedString,
    uuid::Uuid
};
use bevy_ecs::{
    entity::Entity,
    event::Event
};


/// An [`Event`] which is emitted when a player attempts to log in.
#[derive(Event, Debug)]
#[non_exhaustive]
pub struct PlayerRequestLoginEvent {
    /// The [`Entity`] of the peer who is trying to log in.
    pub peer     : Entity,
    /// The UUID of the player who is trying to log in.
    pub uuid     : Uuid,
    /// The username of the player who is trying to log in.
    pub username : BoundedString<16>
}

impl PlayerRequestLoginEvent {
    #[inline]
    pub(crate) fn new(peer : Entity, uuid : Uuid, username : BoundedString<16>) -> Self {
        Self { peer, uuid, username }
    }
}
