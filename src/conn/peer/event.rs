use crate::conn::protocol::packet::c2s::handshake::C2SHandshakePackets;
use std::time::Instant;
use bevy_ecs::{
    entity::Entity,
    event::Event
};


#[derive(Event, Debug)]
pub struct IncomingHandshakePacketEvent {
    pub peer      : Entity,
    pub packet    : C2SHandshakePackets,
    pub timestamp : Instant
}
