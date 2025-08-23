use crate::conn::{
    peer::event::{
        IncomingPacketEvent,
        status::IncomingStatusPacketEvent,
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
use bevy_ecs::event::EventReader;


pub(in crate::conn) fn respond_to_pings(
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
