use crate::game::player::login::PlayerRequestLoginEvent;
use bevy_ecs::{
    entity::Entity,
    event::Event
};


#[derive(Event, Debug)]
pub struct PlayerApproveLoginEvent {
    peer : Entity
}

impl PlayerApproveLoginEvent {
    #[inline(always)]
    pub(in super) fn new(peer : Entity) -> Self {
        Self { peer }
    }
}

impl PlayerApproveLoginEvent {
    #[inline(always)]
    pub fn peer(&self) -> Entity { self.peer }
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
