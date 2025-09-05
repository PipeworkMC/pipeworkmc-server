use crate::conn::protocol::packet::c2s::handshake::C2SHandshakePackets;
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

impl IncomingHandshakePacketEvent {

    #[inline(always)]
    pub fn peer(&self) -> Entity { self.peer }

    #[inline(always)]
    pub fn packet(&self) -> &C2SHandshakePackets { &self.packet }
    #[inline(always)]
    pub fn take_packet(self) -> C2SHandshakePackets { self.packet }

    #[inline(always)]
    pub fn timestamp(&self) -> Instant { self.timestamp }

}
