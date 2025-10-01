use crate::peer::{
    writer::PacketSender,
    event::{
        PacketReceived,
        SendPacket
    }
};
use crate::game::status::StatusRequest;
use pipeworkmc_packet::{
    c2s::{
        C2SPackets,
        status::{
            C2SStatusPackets,
            ping::C2SStatusPingPacket,
            request::C2SStatusRequestPacket
        }
    },
    s2c::status::{
        pong::S2CStatusPongPacket,
        response::S2CStatusResponsePacket
    }
};
use bevy_callback::OptionCallback;
use bevy_ecs::event::{
    EventReader,
    EventWriter
};
use bevy_pareventwriter::ParallelEventWriter;


pub(in crate::peer) fn respond_to_requests(
    mut er_packet : EventReader<PacketReceived>,
    mut ew_packet : EventWriter<SendPacket>,
        c_status  : OptionCallback<StatusRequest>
) {
    if let Some(mut c_status) = c_status.0 {
        for event in er_packet.read() {
            if let C2SPackets::Status(C2SStatusPackets::Request(C2SStatusRequestPacket {})) = &event.packet {
                ew_packet.write(SendPacket::new(event.peer).with_before_switch(
                    S2CStatusResponsePacket::from(c_status.request(StatusRequest { peer : event.peer }) )
                ));
            }
        }
    }
}


/// Automatically responds to status ping requests.
pub(in crate::peer) fn respond_to_pings(
    mut er_packet : EventReader<PacketReceived>,
        ew_packet : ParallelEventWriter<SendPacket>
) {
    er_packet.par_read().for_each(|event| {
        if let C2SPackets::Status(C2SStatusPackets::Ping(C2SStatusPingPacket { timestamp })) = &event.packet {
            ew_packet.write(SendPacket::new(event.peer).with_before_switch(
                S2CStatusPongPacket { timestamp : *timestamp }
            ));
        }
    });
}
