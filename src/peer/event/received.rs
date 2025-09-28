use pipeworkmc_packet::c2s::C2SPackets;
use std::time::Instant;
use bevy_ecs::{
    entity::Entity,
    event::Event
};


/// An [`Event`] which is emitted when the server receives a packet from a peer.
#[derive(Event)]
#[non_exhaustive]
pub struct PacketReceived {
    /// The [`Entity`] of the peer which sent the packet.
    pub entity    : Entity,
    /// The packet which was received.
    pub packet    : C2SPackets,
    /// The timestamp when the packet was received.
    pub timestamp : Instant
}

impl PacketReceived {

    pub(in crate::peer) fn new(entity : Entity, packet : C2SPackets) -> Self {
        Self { entity, packet, timestamp : Instant::now() }
    }

}
