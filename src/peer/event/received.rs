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
    pub peer      : Entity,
    /// The packet which was received.
    pub packet    : C2SPackets,
    /// The timestamp when the packet was received.
    pub timestamp : Instant
}

impl PacketReceived {
    pub(crate) fn new<P>(peer : Entity, packet : P) -> Self
    where
        P : Into<C2SPackets>
    {
        Self { peer, packet : packet.into(), timestamp : Instant::now() }
    }
}
