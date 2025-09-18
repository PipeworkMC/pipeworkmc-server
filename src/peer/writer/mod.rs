use pipeworkmc_codec::{
    encode::{
        EncodeBuf,
        PrefixedPacketEncode
    },
    meta::AtomicPacketState
};
use pipeworkmc_data::redacted::Redacted;
use pipeworkmc_packet::s2c::S2CPackets;
use crate::peer::event::SendPacket;
use core::sync::atomic::{
    AtomicBool,
    Ordering as AtomicOrdering
};
use std::{
    collections::VecDeque,
    net::TcpStream,
    sync::Arc
};
use bevy_ecs::{
    component::Component,
    entity::Entity
};
use openssl::symm::Crypter;


mod write;
pub(in crate::peer) use write::*;

mod sender;
pub use sender::*;


#[derive(Component)]
pub struct PeerStreamWriter {
    stream         : TcpStream,
    encrypter      : Option<Redacted<Crypter>>,
    bytes_to_write : VecDeque<u8>,
    outgoing_state : Arc<AtomicPacketState>,
    disconnecting  : Arc<AtomicBool>
}

impl PeerStreamWriter {

    #[inline(always)]
    pub(in crate::peer) fn new(
        stream         : TcpStream,
        outgoing_state : Arc<AtomicPacketState>,
        disconnecting  : Arc<AtomicBool>
    ) -> Self { Self {
        stream,
        encrypter      : None,
        bytes_to_write : VecDeque::new(),
        outgoing_state,
        disconnecting
    } }

    #[inline(always)]
    pub(in crate::peer) fn set_encrypter(&mut self, encrypter : Redacted<Crypter>) {
        self.encrypter = Some(encrypter);
    }

    pub fn handle_send_packet(&mut self, e : &SendPacket) {
        let prev_state = self.outgoing_state.load(AtomicOrdering::SeqCst);

        if let Some(b) = e.bytes(prev_state) {
            self.bytes_to_write.extend(b);
        } else if let Some(switch_state) = e.switch_state() {
            todo!("switch state from {prev_state:?} to {switch_state:?}");
            if let Some(b) = e.bytes(switch_state) {
                self.bytes_to_write.extend(b);
            }
        }

        if (e.is_kick()) {
            self.disconnecting.store(true, AtomicOrdering::Relaxed);
        }
    }

}


impl PacketSender for &mut PeerStreamWriter {

    fn with<'l, T>(self, packet : T) -> Self
    where
        T : Into<S2CPackets<'l>>
    {
        if (self.disconnecting.load(AtomicOrdering::Relaxed)) { return self; }
        self.handle_send_packet(&SendPacket::new(Entity::PLACEHOLDER).with(packet));
        self
    }

    #[track_caller]
    fn with_nochange<'l, T>(self, packet : T) -> Self
    where
        T : Into<S2CPackets<'l>>
    {
        if (self.disconnecting.load(AtomicOrdering::Relaxed)) { return self; }
        let packet            = packet.into();
        let (state, _, kick,) = packet.meta();
        if (state != self.outgoing_state.load(AtomicOrdering::SeqCst)) {
            panic!("can not send {state:?}");
        }
        if (kick) { self.disconnecting.store(true, AtomicOrdering::Relaxed); }
        let mut buf = EncodeBuf::new(packet.encode_prefixed_len());
        unsafe { packet.encode_prefixed(&mut buf); }
        self.bytes_to_write.extend(buf.as_slice());
        self
    }

}
