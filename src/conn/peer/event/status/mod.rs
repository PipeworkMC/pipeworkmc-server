use crate::conn::{
    peer::event::IncomingPacketEvent,
    protocol::packet::c2s::status::C2SStatusPackets
};
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

impl IncomingPacketEvent for IncomingStatusPacketEvent {
    type Packet = C2SStatusPackets;

    #[inline(always)]
    fn peer(&self) -> Entity { self.peer }

    #[inline(always)]
    fn packet(&self) -> &Self::Packet { &self.packet }
    #[inline(always)]
    fn take_packet(self) -> Self::Packet { self.packet }

    #[inline(always)]
    fn timestamp(&self) -> Instant { self.timestamp }

}
