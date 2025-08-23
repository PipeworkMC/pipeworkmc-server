use crate::conn::{
    peer::event::{
        IncomingPacketEvent,
        status::IncomingStatusPacketEvent,
        OutgoingPacketEvent
    },
    protocol::packet::{
        c2s::status::{
            C2SStatusPackets,
            request::C2SStatusRequestPacket
        },
        s2c::status::response::S2CStatusResponsePacket
    }
};
use crate::util::{
    channel_pair::ChannelPair,
    par_eventwriter::ParallelEventWriter
};
use std::{
    time::Instant,
    sync::mpmc
};
use bevy_ecs::{
    entity::Entity,
    event::{ Event, EventReader, EventWriter }
};


#[derive(Event)]
pub struct StatusRequestEvent {
    peer      : Entity,
    timestamp : Instant,
    sender    : mpmc::Sender<(Entity, S2CStatusResponsePacket,)>
}

impl StatusRequestEvent {

    #[inline(always)]
    pub fn peer(&self) -> Entity { self.peer }

    #[inline(always)]
    pub fn timestamp(&self) -> Instant { self.timestamp }

    #[inline]
    pub fn respond<S>(&self, status : S)
    where
        S : Into<S2CStatusResponsePacket>
    {
        _ = self.sender.send((self.peer, status.into(),));
    }

}


pub(in super::super::super::super) fn respond_to_requests(
    mut er_status  : EventReader<IncomingStatusPacketEvent>,
        ew_request : ParallelEventWriter<StatusRequestEvent>,
    mut ew_packet  : EventWriter<OutgoingPacketEvent>,
        c_status   : ChannelPair<(Entity, S2CStatusResponsePacket,)>
) {

    er_status.par_read().for_each(|event| {
        if let C2SStatusPackets::Request(C2SStatusRequestPacket) = event.packet() {
            ew_request.write(StatusRequestEvent {
                peer      : event.peer(),
                timestamp : event.timestamp(),
                sender    : c_status.sender().clone()
            });
        }
    });

    while let Ok((peer, packet,)) = c_status.receiver().try_recv() {
        ew_packet.write(OutgoingPacketEvent::new(peer, packet));
    }

}
