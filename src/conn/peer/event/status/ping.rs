use crate::conn::peer::{
    ConnPeerSender,
    event::status::IncomingStatusPacketEvent
};
use pipeworkmc_packet::{
    c2s::status::{
        C2SStatusPackets,
        ping::C2SStatusPingPacket
    },
    s2c::status::pong::S2CStatusPongPacket
};
use bevy_ecs::{
    event::EventReader,
    system::Query
};


pub(in crate::conn) fn respond_to_pings(
    mut q_peers   : Query<(&mut ConnPeerSender,)>,
    mut er_status : EventReader<IncomingStatusPacketEvent>
) {
    for event in er_status.read() {
        if let C2SStatusPackets::Ping(C2SStatusPingPacket { timestamp }) = event.packet()
            && let Ok((mut sender,)) = q_peers.get_mut(event.peer())
        {
            sender.send(S2CStatusPongPacket { timestamp : *timestamp });
        }
    }
}
