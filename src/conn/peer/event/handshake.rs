use crate::conn::{
    peer::ConnPeerState,
    protocol::packet::{
        c2s::handshake::{
            C2SHandshakePackets,
            intention::Intention
        }
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
    peer      : Entity,
    packet    : C2SHandshakePackets,
    timestamp : Instant
}

impl IncomingHandshakePacketEvent {

    #[inline]
    pub(crate) fn new(peer : Entity, packet : C2SHandshakePackets) -> Self {
        Self { peer, packet, timestamp : Instant::now() }
    }

    #[inline(always)]
    pub fn peer(&self) -> Entity { self.peer }

    #[inline(always)]
    pub fn packet(&self) -> &C2SHandshakePackets { &self.packet }
    #[inline(always)]
    pub fn take_packet(self) -> C2SHandshakePackets { self.packet }

    #[inline(always)]
    pub fn timestamp(&self) -> Instant { self.timestamp }

}

pub(in super::super::super) fn switch_state(
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
                    todo!("Login");
                    // TODO: Check protocol version.
                },
            };
        }
    }
}
