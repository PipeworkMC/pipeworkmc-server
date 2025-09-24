use crate::peer::{
    writer::PacketSender,
    event::{
        PacketReceived,
        SendPacket
    }
};
use crate::ecs::ParallelEventWriter;
use pipeworkmc_packet::{
    c2s::{
        C2SPackets,
        status::{
            C2SStatusPackets,
            ping::C2SStatusPingPacket
        }
    },
    s2c::status::pong::S2CStatusPongPacket
};
use bevy_ecs::event::EventReader;


pub(in crate::peer) fn respond_to_pings(
    mut er_packet : EventReader<PacketReceived>,
        ew_packet : ParallelEventWriter<SendPacket>
) {
    er_packet.par_read().for_each(|event| {
        if let C2SPackets::Status(C2SStatusPackets::Ping(C2SStatusPingPacket { timestamp })) = event.packet() {
            ew_packet.write(SendPacket::new(event.entity()).with_before_switch(
                S2CStatusPongPacket { timestamp : *timestamp })
            );
        }
    });
}
