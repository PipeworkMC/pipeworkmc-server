use crate::peer::{
    state::PeerState,
    event::PacketReceived
};
use pipeworkmc_packet::c2s::{
    C2SPackets,
    handshake::{
        C2SHandshakePackets,
        intention::{
            C2SHandshakeIntentionPacket,
            Intention
        }
    }
};
use bevy_ecs::{
    event::EventReader,
    system::Query
};


pub(in crate::peer) fn handle_intention(
    mut q_peers   : Query<(&mut PeerState,)>,
    mut er_packet : EventReader<PacketReceived>
) {
    for event in er_packet.read() {
        if let C2SPackets::Handshake(C2SHandshakePackets::Intention(
            C2SHandshakeIntentionPacket { intent, .. }
        )) = event.packet() {
            if let Ok((mut state,)) = q_peers.get_mut(event.entity()) {
                match (intent) {
                    Intention::Status => unsafe { state.switch_to_status() },
                    Intention::Login { .. } => {
                        unsafe { state.switch_to_login(); }
                        // TODO: Check protocol version.
                    },
                };
            }
        }
    }
}
