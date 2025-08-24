use crate::conn::{
    peer::event::login::ConnPeerLoginFlow,
    protocol::packet::{ AtomicPacketState, PacketState }
};
use core::{
    net::SocketAddr,
    time::Duration
};
use std::{
    sync::{ Arc, atomic::Ordering as AtomicOrdering },
    time::Instant
};
use bevy_ecs::{
    bundle::Bundle,
    component::Component
};


mod read_decode;
pub(in crate::conn) use read_decode::*;
mod write_encode;
pub use write_encode::*;

pub mod event;


#[derive(Bundle)]
pub(in crate::conn) struct ConnPeerBundle {
    pub(in crate::conn) peer       : ConnPeer,
    pub(in crate::conn) reader     : ConnPeerReader,
    pub(in crate::conn) incoming   : ConnPeerIncoming,
    pub(in crate::conn) decoder    : ConnPeerDecoder,
    pub(in crate::conn) writer     : ConnPeerWriter,
    pub(in crate::conn) sender     : ConnPeerSender,
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
    outgoing_state : Arc<AtomicPacketState>,
    expires        : Option<Instant> // TODO: Kick on timeout
}

impl ConnPeerState {

    #[inline(always)]
    pub fn incoming_state(&self) -> PacketState { self.incoming_state }

    #[inline(always)]
    pub fn outgoing_state(&self) -> PacketState { self.outgoing_state.load(AtomicOrdering::SeqCst) }

    #[inline(always)]
    pub(in crate::conn) fn outgoing_state_arc(&self) -> &Arc<AtomicPacketState> { &self.outgoing_state }

    #[inline(always)]
    pub fn expires(&self) -> Option<Instant> { self.expires }

}

impl ConnPeerState {

    pub fn handshake() -> Self { Self {
        incoming_state : PacketState::Handshake,
        outgoing_state : Arc::new(AtomicPacketState::new(PacketState::Handshake)),
        expires        : Some(Instant::now() + Duration::from_millis(250))
    } }

    pub unsafe fn switch_to_status(&mut self) {
        self.incoming_state = PacketState::Status;
        self.outgoing_state.store(PacketState::Status, AtomicOrdering::SeqCst);
        self.expires        = Some(Instant::now() + Duration::from_millis(500));
    }
    pub unsafe fn switch_to_login(&mut self) {
        self.incoming_state = PacketState::Login;
        self.outgoing_state.store(PacketState::Login, AtomicOrdering::SeqCst);
        self.expires        = Some(Instant::now() + Duration::from_millis(2500));
    }

    pub unsafe fn login_finish(&mut self) {
        self.outgoing_state.store(PacketState::Config, AtomicOrdering::SeqCst);
        self.expires = Some(Instant::now() + Duration::from_millis(250));
    }
    pub unsafe fn login_finish_acknowledged(&mut self) {
        self.incoming_state = PacketState::Config;
        self.expires        = None;
    }

    pub unsafe fn config_finish(&mut self) {
        self.outgoing_state.store(PacketState::Play, AtomicOrdering::SeqCst);
        self.expires = Some(Instant::now() + Duration::from_millis(250));
    }
    pub unsafe fn config_finish_acknowledged(&mut self) {
        self.incoming_state = PacketState::Play;
        self.expires        = None;
    }

    pub unsafe fn config_begin(&mut self) {
        self.outgoing_state.store(PacketState::Config, AtomicOrdering::SeqCst);
        self.expires = Some(Instant::now() + Duration::from_millis(500));
    }
    pub unsafe fn config_begin_acknowledged(&mut self) {
        self.incoming_state = PacketState::Config;
        self.expires        = None;
    }

}
