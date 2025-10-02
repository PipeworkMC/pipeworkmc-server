use pipeworkmc_data::{
    bounded_string::BoundedString,
    uuid::Uuid
};
use bevy_ecs::{
    entity::Entity,
    message::Message
};


/// An [`Message`] which is emitted when a player successfully logs in.
#[derive(Message, Debug)]
#[non_exhaustive]
pub struct PlayerLoggedInMessage {
    /// The [`Entity`] of the peer who logged in.
    pub peer     : Entity,
    /// The UUID of the player who logged in.
    pub uuid     : Uuid,
    /// The username of the player who logged in.
    pub username : BoundedString<16>
}
