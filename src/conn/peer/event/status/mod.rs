use crate::conn::protocol::packet::c2s::status::C2SStatusPackets;
use std::time::Instant;
use bevy_ecs::{
    entity::Entity,
    event::Event
};


mod ping;
pub(in crate::conn) use ping::*;


#[derive(Event, Debug)]
pub struct IncomingStatusPacketEvent {
    peer      : Entity,
    packet    : C2SStatusPackets,
    timestamp : Instant
}

impl IncomingStatusPacketEvent {
    #[inline]
    pub(crate) fn new(peer : Entity, packet : C2SStatusPackets) -> Self {
        Self { peer, packet, timestamp : Instant::now() }
    }
}

impl IncomingStatusPacketEvent {

    #[inline(always)]
    pub fn peer(&self) -> Entity { self.peer }

    #[inline(always)]
    pub fn packet(&self) -> &C2SStatusPackets { &self.packet }
    #[inline(always)]
    pub fn take_packet(self) -> C2SStatusPackets { self.packet }

    #[inline(always)]
    pub fn timestamp(&self) -> Instant { self.timestamp }

}
