use crate::conn::{
    peer::event::login::ConnPeerLoginFlow,
    protocol::packet::PacketState
};
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
pub(in crate::conn) use read_decode::*;
mod write_encode;
pub(in crate::conn) use write_encode::*;

pub mod event;


#[derive(Bundle)]
pub(in crate::conn) struct ConnPeerBundle {
    pub(in crate::conn) peer       : ConnPeer,
    pub(in crate::conn) reader     : ConnPeerReader,
    pub(in crate::conn) incoming   : ConnPeerIncoming,
    pub(in crate::conn) decoder    : ConnPeerDecoder,
    pub(in crate::conn) writer     : ConnPeerWriter,
    pub(in crate::conn) outgoing   : ConnPeerOutgoing,
    pub(in crate::conn) state      : ConnPeerState,
    pub(in crate::conn) login_flow : ConnPeerLoginFlow
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
pub struct ConnPeerState {
    incoming_state : PacketState,
    outgoing_state : PacketState,
    expires        : Option<Instant> // TODO: Kick on timeout
}
impl ConnPeerState {

    pub fn handshake() -> Self { Self {
        incoming_state : PacketState::Handshake,
        outgoing_state : PacketState::Handshake,
        expires        : Some(Instant::now() + Duration::from_millis(250))
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

    pub fn login_finish(&mut self) {
        self.outgoing_state = PacketState::Config;
        self.expires        = Some(Instant::now() + Duration::from_millis(250));
    }
    pub fn login_finish_acknowledged(&mut self) {
        self.incoming_state = PacketState::Config;
        self.expires        = None;
    }

    pub fn config_finish(&mut self) {
        self.outgoing_state = PacketState::Play;
        self.expires        = Some(Instant::now() + Duration::from_millis(250));
    }
    pub fn config_finish_acknowledged(&mut self) {
        self.incoming_state = PacketState::Play;
        self.expires        = None;
    }

    pub fn config_begin(&mut self) {
        self.outgoing_state = PacketState::Config;
        self.expires        = Some(Instant::now() + Duration::from_millis(500));
    }
    pub fn config_begin_acknowledged(&mut self) {
        self.incoming_state = PacketState::Config;
        self.expires        = None;
    }

}
