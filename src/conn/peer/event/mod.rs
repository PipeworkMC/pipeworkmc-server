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
