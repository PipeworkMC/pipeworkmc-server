use crate::conn::protocol::packet::s2c::S2CPackets;
use core::fmt::Debug;
use std::time::Instant;
use bevy_ecs::{
    entity::Entity,
    event::Event
};


pub mod handshake;
pub mod status;
pub mod login;


pub trait IncomingPacketEvent
where
    Self : Event + Debug
{
    type Packet;

    fn peer(&self) -> Entity;

    fn packet(&self) -> &Self::Packet;
    fn take_packet(self) -> Self::Packet;

    fn timestamp(&self) -> Instant;

}


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
