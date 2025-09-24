use crate::game::login::PlayerRequestLoginEvent;
use bevy_ecs::{
    entity::Entity,
    event::Event
};


#[derive(Event, Debug)]
pub struct PlayerApproveLoginEvent {
    entity : Entity
}

impl PlayerApproveLoginEvent {
    #[inline(always)]
    pub(in super) fn new(entity : Entity) -> Self {
        Self { entity }
    }
}

impl PlayerApproveLoginEvent {
    #[inline(always)]
    pub fn entity(&self) -> Entity { self.entity }
}

impl From<&PlayerRequestLoginEvent> for PlayerApproveLoginEvent {
    #[inline]
    fn from(value : &PlayerRequestLoginEvent) -> Self {
        Self::new(value.peer())
    }
}
impl From<PlayerRequestLoginEvent> for PlayerApproveLoginEvent {
    #[inline(always)]
    fn from(value : PlayerRequestLoginEvent) -> Self { Self::from(&value) }
}
