use pipeworkmc_packet::c2s::play::C2SPlayPackets;
use std::time::Instant;
use bevy_ecs::{
    entity::Entity,
    event::Event
};


mod keepalive;
pub(in crate::conn) use keepalive::*;


#[derive(Event, Debug)]
pub struct IncomingPlayPacketEvent {
    peer      : Entity,
    packet    : C2SPlayPackets,
    timestamp : Instant
}

impl IncomingPlayPacketEvent {
    #[inline]
    pub(crate) fn new(peer : Entity, packet : C2SPlayPackets) -> Self {
        Self { peer, packet, timestamp : Instant::now() }
    }
}

impl IncomingPlayPacketEvent {

    #[inline(always)]
    pub fn peer(&self) -> Entity { self.peer }

    #[inline(always)]
    pub fn packet(&self) -> &C2SPlayPackets { &self.packet }
    #[inline(always)]
    pub fn take_packet(self) -> C2SPlayPackets { self.packet }

    #[inline(always)]
    pub fn timestamp(&self) -> Instant { self.timestamp }

}
