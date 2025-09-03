use crate::conn::{
    peer::event::IncomingPacketEvent,
    protocol::{
        packet::{
            c2s::login::C2SLoginPackets
        },
        value::bounded_string::BoundedString
    }
};
use std::time::Instant;
use bevy_ecs::{
    entity::Entity,
    event::Event
};
use uuid::Uuid;


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

impl IncomingPacketEvent for IncomingLoginPacketEvent {
    type Packet = C2SLoginPackets;

    #[inline(always)]
    fn peer(&self) -> Entity { self.peer }

    #[inline(always)]
    fn packet(&self) -> &Self::Packet { &self.packet }
    #[inline(always)]
    fn take_packet(self) -> Self::Packet { self.packet }

    #[inline(always)]
    fn timestamp(&self) -> Instant { self.timestamp }

}


#[derive(Event, Debug)]
pub struct LoggedInEvent {
    peer     : Entity,
    uuid     : Uuid,
    username : BoundedString<16>
}

impl LoggedInEvent {
    #[inline(always)]
    pub fn peer(&self) -> Entity { self.peer }

    #[inline(always)]
    pub fn uuid(&self) -> Uuid { self.uuid }
    #[inline(always)]
    pub fn username(&self) -> &BoundedString<16> { &self.username }
}
