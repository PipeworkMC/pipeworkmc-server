use crate::conn::{
    peer::event::IncomingPacketEvent,
    protocol::packet::c2s::handshake::C2SHandshakePackets
};
use std::time::Instant;
use bevy_ecs::{
    entity::Entity,
    event::Event
};


mod intention;
pub(in crate::conn) use intention::*;


#[derive(Event, Debug)]
pub struct IncomingHandshakePacketEvent {
    peer      : Entity,
    packet    : C2SHandshakePackets,
    timestamp : Instant
}

impl IncomingHandshakePacketEvent {
    #[inline]
    pub(crate) fn new(peer : Entity, packet : C2SHandshakePackets) -> Self {
        Self { peer, packet, timestamp : Instant::now() }
    }
}

impl IncomingPacketEvent for IncomingHandshakePacketEvent {
    type Packet = C2SHandshakePackets;

    #[inline(always)]
    fn peer(&self) -> Entity { self.peer }

    #[inline(always)]
    fn packet(&self) -> &Self::Packet { &self.packet }
    #[inline(always)]
    fn take_packet(self) -> Self::Packet { self.packet }

    #[inline(always)]
    fn timestamp(&self) -> Instant { self.timestamp }

}
