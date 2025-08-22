use crate::conn::{
    peer::ConnPeerState,
    protocol::packet::c2s::{
        handshake::{
            C2SHandshakePackets,
            intention::Intention
        },
        status::C2SStatusPackets
    }
};
use std::time::Instant;
use bevy_ecs::{
    entity::Entity,
    event::{ Event, EventReader },
    system::Query
};


#[derive(Event, Debug)]
pub struct IncomingHandshakePacketEvent {
    pub peer      : Entity,
    pub packet    : C2SHandshakePackets,
    pub timestamp : Instant
}

pub(in super::super) fn switch_handshake_state(
    mut q_peers      : Query<(&mut ConnPeerState,)>,
    mut er_handshake : EventReader<IncomingHandshakePacketEvent>
) {
    for e in er_handshake.read() {
        let C2SHandshakePackets::Intention(intention) = &e.packet;
        if let Ok((mut state,)) = q_peers.get_mut(e.peer) {
            match (intention.intent) {
                Intention::Status => state.switch_to_status(),
                Intention::Login { .. } => {
                    state.switch_to_login();
                    todo!("Login");
                    // TODO: Check protocol version.
                },
            };
        }
    }
}


#[derive(Event, Debug)]
pub struct IncomingStatusPacketEvent {
    pub peer      : Entity,
    pub packet    : C2SStatusPackets,
    pub timestamp : Instant
}
