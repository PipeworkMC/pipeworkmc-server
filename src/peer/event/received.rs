use pipeworkmc_packet::c2s::C2SPackets;
use std::time::Instant;
use bevy_ecs::{
    entity::Entity,
    event::Event
};


#[derive(Event)]
pub struct PacketReceived {
    entity    : Entity,
    packet    : C2SPackets,
    timestamp : Instant
}

impl PacketReceived {

    #[inline(always)]
    pub fn entity(&self) -> Entity { self.entity }

    #[inline(always)]
    pub fn packet(&self) -> &C2SPackets { &self.packet }

    #[inline(always)]
    pub fn take_packet(self) -> C2SPackets { self.packet }

    #[inline(always)]
    pub fn timestamp(&self) -> Instant { self.timestamp }

}

impl PacketReceived {

    pub(in crate::peer) fn new(entity : Entity, packet : C2SPackets) -> Self {
        Self { entity, packet, timestamp : Instant::now() }
    }

}
