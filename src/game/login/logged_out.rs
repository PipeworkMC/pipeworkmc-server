use pipeworkmc_data::{
    bounded_string::BoundedString,
    uuid::Uuid
};
use bevy_ecs::{
    entity::Entity,
    event::Event
};


/// An [`Event`] which is emitted when a player logs out.
///
/// *Note: [`PlayerLoggedInEvent`](super::PlayerLoggedInEvent)* does not need to have been
///  emitted for [`PlayerLoggedOutEvent`] to be emitted.
#[derive(Event, Debug)]
#[non_exhaustive]
pub struct PlayerLoggedOutEvent {
    /// The [`Entity`] of the peer who logged out.
    pub peer     : Entity,
    /// The UUID of the player who logged out.
    pub uuid     : Uuid,
    /// The username of the player who logged out.
    pub username : BoundedString<16>
}

impl PlayerLoggedOutEvent {
    #[inline(always)]
    pub(crate) fn new(peer : Entity, uuid : Uuid, username : BoundedString<16>) -> Self {
        Self { peer, uuid, username }
    }
}
