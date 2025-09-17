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


#[derive(Component, Debug)]
pub struct PeerState {
    incoming      : PacketState,
    outgoing      : Arc<AtomicPacketState>,
    expires       : Instant,
    disconnecting : Arc<AtomicBool>
}

impl PeerState {

    #[inline(always)]
    pub fn incoming(&self) -> PacketState { self.incoming }

    #[inline(always)]
    pub fn outgoing(&self) -> PacketState { self.outgoing.load(AtomicOrdering::SeqCst) }

    #[inline(always)]
    pub fn expires(&self) -> Instant { self.expires }

    #[inline(always)]
    pub fn disconnecting(&self) -> bool { self.disconnecting.load(AtomicOrdering::Relaxed) }

    #[inline(always)]
    pub(in crate::peer) fn disconnect(&mut self) { self.disconnecting.store(true, AtomicOrdering::Relaxed); }

}

pub(in crate::peer) const KEEPALIVE_TIMEOUT : Duration = Duration::from_secs(5);

impl PeerState {

    pub(in crate::peer) unsafe fn new(
        outgoing_state : Arc<AtomicPacketState>,
        disconnecting  : Arc<AtomicBool>
    ) -> Self { Self {
        incoming      : PacketState::Handshake,
        outgoing      : outgoing_state,
        expires       : Instant::now() + Duration::from_millis(250),
        disconnecting
    } }

    pub unsafe fn switch_to_status(&mut self) {
        self.incoming = PacketState::Status;
        self.outgoing.store(PacketState::Status, AtomicOrdering::SeqCst);
        self.expires  = Instant::now() + Duration::from_millis(500);
    }
    pub unsafe fn switch_to_login(&mut self) {
        self.incoming = PacketState::Login;
        self.outgoing.store(PacketState::Login, AtomicOrdering::SeqCst);
        self.expires  = Instant::now() + Duration::from_millis(2500);
    }

    pub unsafe fn login_finish(&mut self) {
        self.outgoing.store(PacketState::Config, AtomicOrdering::SeqCst);
        self.expires = Instant::now() + Duration::from_millis(250);
    }
    pub unsafe fn login_finish_acknowledged(&mut self) {
        self.incoming = PacketState::Config;
        self.expires  = Instant::now() + KEEPALIVE_TIMEOUT;
    }

    pub unsafe fn config_finish(&mut self) {
        self.outgoing.store(PacketState::Play, AtomicOrdering::SeqCst);
        self.expires = Instant::now() + Duration::from_millis(250);
    }
    pub unsafe fn config_finish_acknowledged(&mut self) {
        self.incoming = PacketState::Play;
        self.expires  = Instant::now() + KEEPALIVE_TIMEOUT;
    }

    pub unsafe fn config_begin(&mut self) {
        self.outgoing.store(PacketState::Config, AtomicOrdering::SeqCst);
        self.expires = Instant::now() + Duration::from_millis(500);
    }
    pub unsafe fn config_begin_acknowledged(&mut self) {
        self.incoming = PacketState::Config;
        self.expires  = Instant::now() + KEEPALIVE_TIMEOUT;
    }

}
