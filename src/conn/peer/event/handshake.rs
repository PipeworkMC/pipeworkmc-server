use crate::conn::{
    peer::{
        ConnPeerState,
        event::IncomingPacketEvent
    },
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
}

impl IncomingPacketEvent for IncomingHandshakePacketEvent {
    type Packet = C2SHandshakePackets;

    #[inline(always)]
    fn peer(&self) -> Entity { self.peer }

    #[inline(always)]
    fn packet(&self) -> &Self::Packet { &self.packet }
    #[inline(always)]
    fn take_packet(self) -> Self::Packet { self.packet }

    #[inline(always)]
    fn timestamp(&self) -> Instant { self.timestamp }

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
                    // TODO: Check protocol version.
                },
            };
        }
    }
}
