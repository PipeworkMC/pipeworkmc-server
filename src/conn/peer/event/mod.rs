use crate::conn::protocol::packet::s2c::S2CPackets;
use std::time::Instant;
use bevy_ecs::{
    entity::Entity,
    event::Event
};


pub mod handshake;

pub mod status;


#[derive(Event, Debug)]
pub struct OutgoingPacketEvent {
    peer      : Entity,
    packet    : S2CPackets,
    timestamp : Instant
}

impl OutgoingPacketEvent {

    #[inline]
    pub fn new<P>(peer : Entity, packet : P) -> Self
    where
        P : Into<S2CPackets>
    { Self {
        peer,
        packet    : packet.into(),
        timestamp : Instant::now()
    } }

    #[inline(always)]
    pub fn peer(&self) -> Entity { self.peer }

    #[inline(always)]
    pub fn packet(&self) -> &S2CPackets { &self.packet }

    #[inline(always)]
    pub fn timestamp(&self) -> Instant { self.timestamp }

}
