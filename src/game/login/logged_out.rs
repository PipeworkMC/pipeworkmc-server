use pipeworkmc_data::{
    bounded_string::BoundedString,
    uuid::Uuid
};
use bevy_ecs::{
    entity::Entity,
    message::Message
};


/// An [`Message`] which is emitted when a player logs out.
///
/// *Note: [`PlayerLoggedInMessage`](super::PlayerLoggedInMessage)* does not need to have been
///  emitted for [`PlayerLoggedOutMessage`] to be emitted.
#[derive(Message, Debug)]
#[non_exhaustive]
pub struct PlayerLoggedOutMessage {
    /// The [`Entity`] of the peer who logged out.
    pub peer     : Entity,
    /// The UUID of the player who logged out.
    pub uuid     : Uuid,
    /// The username of the player who logged out.
    pub username : BoundedString<16>
}
