use crate::peer::{
    writer::PacketSender,
    message::{
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
use bevy_callback::Callback;
use bevy_ecs::message::{
    MessageReader,
    MessageWriter
};
use bevy_parmessagewriter::ParallelMessageWriter;


pub(in crate::peer) fn respond_to_requests(
    mut mr_packet : MessageReader<PacketReceived>,
    mut mw_packet : MessageWriter<SendPacket>,
    mut c_status  : Callback<StatusRequest>
) {
    for m in mr_packet.read() {
        if let C2SPackets::Status(C2SStatusPackets::Request(C2SStatusRequestPacket {})) = &m.packet {
            mw_packet.write(SendPacket::new(m.peer).with_before_switch(
                S2CStatusResponsePacket::from(c_status.request(StatusRequest { peer : m.peer }) )
            ));
        }
    }
}


/// Automatically responds to status ping requests.
pub(in crate::peer) fn respond_to_pings(
    mut mr_packet : MessageReader<PacketReceived>,
        mw_packet : ParallelMessageWriter<SendPacket>
) {
    mr_packet.par_read().for_each(|m| {
        if let C2SPackets::Status(C2SStatusPackets::Ping(C2SStatusPingPacket { timestamp })) = &m.packet {
            mw_packet.write(SendPacket::new(m.peer).with_before_switch(
                S2CStatusPongPacket { timestamp : *timestamp }
            ));
        }
    });
}
