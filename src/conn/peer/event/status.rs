use crate::conn::{
    peer::event::{
        IncomingPacketEvent,
        OutgoingPacketEvent
    },
    protocol::packet::{
        c2s::status::{
            C2SStatusPackets,
            ping::C2SStatusPingPacket
        },
        s2c::status::pong::S2CStatusPongPacket
    }
};
use crate::util::par_eventwriter::ParallelEventWriter;
use std::time::Instant;
use bevy_ecs::{
    entity::Entity,
    event::{
        Event,
        EventReader
    }
};


#[derive(Event, Debug)]
pub struct IncomingStatusPacketEvent {
    peer      : Entity,
    packet    : C2SStatusPackets,
    timestamp : Instant
}

impl IncomingStatusPacketEvent {
    #[inline]
    pub(crate) fn new(peer : Entity, packet : C2SStatusPackets) -> Self {
        Self { peer, packet, timestamp : Instant::now() }
    }
}

impl IncomingPacketEvent for IncomingStatusPacketEvent {
    type Packet = C2SStatusPackets;

    #[inline(always)]
    fn peer(&self) -> Entity { self.peer }

    #[inline(always)]
    fn packet(&self) -> &Self::Packet { &self.packet }
    #[inline(always)]
    fn take_packet(self) -> Self::Packet { self.packet }

    #[inline(always)]
    fn timestamp(&self) -> Instant { self.timestamp }

}


pub(in super::super::super) fn send_pong(
    mut er_status : EventReader<IncomingStatusPacketEvent>,
        ew_packet : ParallelEventWriter<OutgoingPacketEvent>
) {
    er_status.par_read().for_each(|event| {
        if let C2SStatusPackets::Ping(C2SStatusPingPacket { timestamp }) = event.packet() {
            ew_packet.write(OutgoingPacketEvent::new(event.peer(),
                S2CStatusPongPacket { timestamp : *timestamp }
            ));
        }
    });
}
