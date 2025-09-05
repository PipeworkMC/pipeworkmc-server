use crate::conn::{
    peer::event::IncomingPacketEvent,
    protocol::packet::c2s::play::C2SPlayPackets
};
use std::time::Instant;
use bevy_ecs::{
    entity::Entity,
    event::Event
};


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

impl IncomingPacketEvent for IncomingPlayPacketEvent {
    type Packet = C2SPlayPackets;

    #[inline(always)]
    fn peer(&self) -> Entity { self.peer }

    #[inline(always)]
    fn packet(&self) -> &Self::Packet { &self.packet }
    #[inline(always)]
    fn take_packet(self) -> Self::Packet { self.packet }

    #[inline(always)]
    fn timestamp(&self) -> Instant { self.timestamp }

}
