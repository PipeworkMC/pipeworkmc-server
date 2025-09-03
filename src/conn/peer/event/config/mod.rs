use crate::conn::{
    peer::event::IncomingPacketEvent,
    protocol::packet::{
        c2s::config::C2SConfigPackets
    }
};
use std::time::Instant;
use bevy_ecs::{
    entity::Entity,
    event::Event
};


#[derive(Event, Debug)]
pub struct IncomingConfigPacketEvent {
    peer      : Entity,
    packet    : C2SConfigPackets,
    timestamp : Instant
}

impl IncomingConfigPacketEvent {
    #[inline]
    pub(crate) fn new(peer : Entity, packet : C2SConfigPackets) -> Self {
        Self { peer, packet, timestamp : Instant::now() }
    }
}

impl IncomingPacketEvent for IncomingConfigPacketEvent {
    type Packet = C2SConfigPackets;

    #[inline(always)]
    fn peer(&self) -> Entity { self.peer }

    #[inline(always)]
    fn packet(&self) -> &Self::Packet { &self.packet }
    #[inline(always)]
    fn take_packet(self) -> Self::Packet { self.packet }

    #[inline(always)]
    fn timestamp(&self) -> Instant { self.timestamp }

}
