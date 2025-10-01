use pipeworkmc_data::{
    bounded_string::BoundedString,
    text::Text,
    uuid::Uuid
};
use bevy_callback::Request;
use bevy_ecs::entity::Entity;


/// A request which is emitted when a player attempts to log in.
#[derive(Debug)]
#[non_exhaustive]
pub struct PlayerLoginRequest {
    /// The [`Entity`] of the peer who is trying to log in.
    pub peer     : Entity,
    /// The UUID of the player who is trying to log in.
    pub uuid     : Uuid,
    /// The username of the player who is trying to log in.
    pub username : BoundedString<16>
}

impl Request for PlayerLoginRequest {
    type Response = Result<(), Text>;
}
