use crate::conn::protocol::packet::PacketState;
use core::{
    net::SocketAddr,
    time::Duration
};
use std::time::Instant;
use bevy_ecs::{
    bundle::Bundle,
    component::Component
};


mod read_decode;
pub(super) use read_decode::*;
mod write_encode;
pub(super) use write_encode::*;

pub mod event;


#[derive(Bundle)]
pub(super) struct ConnPeerBundle {
    pub(super) peer     : ConnPeer,
    pub(super) reader   : ConnPeerReader,
    pub(super) incoming : ConnPeerIncoming,
    pub(super) decoder  : ConnPeerDecoder,
    pub(super) writer   : ConnPeerWriter,
    pub(super) outgoing : ConnPeerOutgoing,
    pub(super) state    : ConnPeerState
}


#[derive(Component)]
pub struct ConnPeer {
    #[expect(dead_code)]
    addr : SocketAddr
}

impl From<SocketAddr> for ConnPeer {
    #[inline(always)]
    fn from(addr : SocketAddr) -> Self { Self { addr } }
}


#[derive(Component)]
pub struct ConnPeerState { // TODO: Timeout handshake/status/login
    incoming_state : PacketState,
    outgoing_state : PacketState,
    expires        : Option<Instant>
}
impl ConnPeerState {

    pub fn handshake() -> Self { Self {
        incoming_state : PacketState::Handshake,
        outgoing_state : PacketState::Handshake,
        expires        : Some(Instant::now() + Duration::from_millis(500))
    } }

    pub fn switch_to_status(&mut self) {
        self.incoming_state = PacketState::Status;
        self.outgoing_state = PacketState::Status;
        self.expires        = Some(Instant::now() + Duration::from_millis(500));
    }

    pub fn switch_to_login(&mut self) {
        self.incoming_state = PacketState::Login;
        self.outgoing_state = PacketState::Login;
        self.expires        = Some(Instant::now() + Duration::from_millis(2500));
    }

}
