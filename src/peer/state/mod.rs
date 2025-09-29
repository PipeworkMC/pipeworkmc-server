use pipeworkmc_codec::meta::{
    PacketState,
    AtomicPacketState
};
use core::{
    sync::atomic::{
        AtomicBool,
        Ordering as AtomicOrdering
    },
    time::Duration
};
use std::{
    sync::Arc,
    time::Instant
};
use bevy_ecs::component::Component;


mod timeout;
pub(in crate::peer) use timeout::*;


/// The current state of a peer.
#[derive(Component, Debug)]
pub struct PeerState {
    incoming      : PacketState,
    outgoing      : Arc<AtomicPacketState>,
    expires       : Option<Instant>,
    disconnecting : Arc<AtomicBool>
}

impl PeerState {

    /// The incoming packet state of the peer.
    /// This is what state to interpret serverbound packets as.
    #[inline]
    pub fn incoming(&self) -> PacketState { self.incoming }

    /// The outgoing packet state of the peer.
    /// This is what state to send clientbound packets as.
    #[inline]
    pub fn outgoing(&self) -> PacketState { self.outgoing.load(AtomicOrdering::SeqCst) }

    /// When the current state will expire, kicking the peer.
    #[inline]
    pub fn expires(&self) -> Option<Instant> { self.expires }

    /// Whether this peer is currently disconnecting.
    #[inline]
    pub fn disconnecting(&self) -> bool { self.disconnecting.load(AtomicOrdering::Relaxed) }

}

impl PeerState {

    pub(in crate::peer) fn new(
        outgoing_state : Arc<AtomicPacketState>,
        disconnecting  : Arc<AtomicBool>
    ) -> Self { Self {
        incoming      : PacketState::Handshake,
        outgoing      : outgoing_state,
        expires       : Some(Instant::now() + Duration::from_millis(250)),
        disconnecting
    } }

    pub(in crate::peer) fn both_to_status(&mut self) {
        self.incoming = PacketState::Status;
        self.outgoing.store(PacketState::Status, AtomicOrdering::SeqCst);
        self.expires  = Some(Instant::now() + Duration::from_millis(500));
    }

    pub(in crate::peer) fn both_to_login(&mut self) {
        self.incoming = PacketState::Login;
        self.outgoing.store(PacketState::Login, AtomicOrdering::SeqCst);
        self.expires  = Some(Instant::now() + Duration::from_millis(2500));
    }

    pub (in crate::peer) fn incoming_to_config(&mut self) {
        self.incoming = PacketState::Config;
        self.expires  = None;
    }

    pub (in crate::peer) fn incoming_to_play(&mut self) {
        self.incoming = PacketState::Play;
        self.expires  = None;
    }

    // pub(in crate::peer) fn login_finish(&mut self) {
    //     self.outgoing.store(PacketState::Config, AtomicOrdering::SeqCst);
    //     self.expires = Instant::now() + Duration::from_millis(250);
    // }
    // pub(in crate::peer) fn login_finish_acknowledged(&mut self) {
    //     self.incoming = PacketState::Config;
    //     self.expires  = Instant::now() + KEEPALIVE_TIMEOUT;
    // }

    // pub(in crate::peer) fn config_finish(&mut self) {
    //     self.outgoing.store(PacketState::Play, AtomicOrdering::SeqCst);
    //     self.expires = Instant::now() + Duration::from_millis(250);
    // }
    // pub(in crate::peer) fn config_finish_acknowledged(&mut self) {
    //     self.incoming = PacketState::Play;
    //     self.expires  = Instant::now() + KEEPALIVE_TIMEOUT;
    // }

    // pub(in crate::peer) fn config_begin(&mut self) {
    //     self.outgoing.store(PacketState::Config, AtomicOrdering::SeqCst);
    //     self.expires = Instant::now() + Duration::from_millis(500);
    // }
    // pub(in crate::peer) fn config_begin_acknowledged(&mut self) {
    //     self.incoming = PacketState::Config;
    //     self.expires  = Instant::now() + KEEPALIVE_TIMEOUT;
    // }

}
