use crate::game::login::PlayerRequestLoginEvent;
use bevy_ecs::{
    entity::Entity,
    event::Event
};


/// An [`Event`] which can be emitted to approve a player login.
#[derive(Event, Debug)]
pub struct PlayerApproveLoginEvent {
    entity : Entity
}

impl PlayerApproveLoginEvent {
    #[inline]
    pub(in super) fn new(entity : Entity) -> Self {
        Self { entity }
    }
}

impl PlayerApproveLoginEvent {
    /// The [`Entity`] of the peer to approve.
    #[inline]
    pub fn entity(&self) -> Entity { self.entity }
}

impl From<&PlayerRequestLoginEvent> for PlayerApproveLoginEvent {
    #[inline]
    fn from(value : &PlayerRequestLoginEvent) -> Self {
        Self::new(value.peer)
    }
}
impl From<PlayerRequestLoginEvent> for PlayerApproveLoginEvent {
    #[inline]
    fn from(value : PlayerRequestLoginEvent) -> Self { Self::from(&value) }
}
