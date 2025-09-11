use pipeworkmc_packet::c2s::login::C2SLoginPackets;
use std::time::Instant;
use bevy_ecs::{
    entity::Entity,
    event::Event
};


mod flow;
pub(in crate::conn) use flow::*;


#[derive(Event, Debug)]
pub struct IncomingLoginPacketEvent {
    peer      : Entity,
    packet    : C2SLoginPackets,
    timestamp : Instant
}

impl IncomingLoginPacketEvent {
    #[inline]
    pub(crate) fn new(peer : Entity, packet : C2SLoginPackets) -> Self {
        Self { peer, packet, timestamp : Instant::now() }
    }
}

impl IncomingLoginPacketEvent {

    #[inline(always)]
    pub fn peer(&self) -> Entity { self.peer }

    #[inline(always)]
    pub fn packet(&self) -> &C2SLoginPackets { &self.packet }
    #[inline(always)]
    pub fn take_packet(self) -> C2SLoginPackets { self.packet }

    #[inline(always)]
    pub fn timestamp(&self) -> Instant { self.timestamp }

}
