//! Status flow data and systems.


pub use pipeworkmc_packet::s2c::status::response::{
    Status, StatusVersion, StatusPlayers, StatusPlayer, StatusPlayerName
};
use bevy_callback::Request;
use bevy_ecs::entity::Entity;


mod auto;
pub use auto::*;


/// A request which is emitted when the server status is requested.
#[derive(Debug)]
#[non_exhaustive]
pub struct StatusRequest {
    /// The [`Entity`] of the requesting peer.
    pub peer : Entity
}

impl Request for StatusRequest {
    type Response = Status<'static>;
}
