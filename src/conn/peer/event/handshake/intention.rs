use crate::conn::{
    peer::{
        ConnPeerState,
        event::handshake::IncomingHandshakePacketEvent
    },
    protocol::packet::c2s::handshake::{
        C2SHandshakePackets,
        intention::Intention
    }
};
use bevy_ecs::{
    event::EventReader,
    system::Query
};


pub(in crate::conn) fn handle_intention(
    mut q_peers      : Query<(&mut ConnPeerState,)>,
    mut er_handshake : EventReader<IncomingHandshakePacketEvent>
) {
    for event in er_handshake.read() {
        let C2SHandshakePackets::Intention(intention) = &event.packet;
        if let Ok((mut state,)) = q_peers.get_mut(event.peer) {
            match (intention.intent) {
                Intention::Status => state.switch_to_status(),
                Intention::Login { .. } => {
                    state.switch_to_login();
                    // TODO: Check protocol version.
                },
            };
        }
    }
}
