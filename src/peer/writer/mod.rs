use pipeworkmc_codec::{
    encode::{
        EncodeBuf,
        PrefixedPacketEncode
    },
    meta::{
        PacketState,
        AtomicPacketState
    }
};
use pipeworkmc_data::redacted::Redacted;
use pipeworkmc_packet::s2c::{
    S2CPackets,
    config::finish::S2CConfigFinishPacket
};
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
pub(in crate::peer) struct PeerStreamWriter {
    stream         : TcpStream,
    encrypter      : Option<Redacted<Crypter>>,
    bytes_to_write : VecDeque<u8>,
    outgoing_state : Arc<AtomicPacketState>,
    disconnecting  : Arc<AtomicBool>
}

impl PeerStreamWriter {

    #[inline]
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

    #[inline]
    pub(in crate::peer) fn set_encrypter(&mut self, encrypter : Redacted<Crypter>) {
        self.encrypter = Some(encrypter);
    }

    fn handle_send_packet(&mut self, e : &SendPacket) {
        if (self.disconnecting.load(AtomicOrdering::Relaxed)) { return; }

        let old_state = self.outgoing_state.load(AtomicOrdering::SeqCst);
        // println!("sending packet from {}:{}:{} handshake={:?} status={:?} login={:?} config={:?} play={:?} {old_state:?}", // TODO: Remove
        //     e.sent_by().file(), e.sent_by().line(), e.sent_by().column(),
        //     e.before_switch(PacketState::Handshake).is_some(),
        //     e.before_switch(PacketState::Status).is_some(),
        //     e.before_switch(PacketState::Login).is_some(),
        //     e.before_switch(PacketState::Config).is_some(),
        //     e.before_switch(PacketState::Play).is_some()
        // );
        if let Some(b) = e.before_switch(old_state) {
            self.bytes_to_write.extend(b);
        }

        if let Some((new_state, b, skip_intermediate,)) = e.after_switch() {
            if (skip_intermediate) {
                self.outgoing_state.store(new_state, AtomicOrdering::SeqCst);
            } else {
                match ((old_state, new_state,)) {
                    (PacketState::Handshake, PacketState::Handshake, ) => { },
                    (PacketState::Status,    PacketState::Status,    ) => { },
                    (PacketState::Login,     PacketState::Login,     ) => { },
                    (PacketState::Config,    PacketState::Config,    ) => { },
                    (PacketState::Play,      PacketState::Play,      ) => { },
                    (PacketState::Config,    PacketState::Play,      ) => {
                        let     packet = S2CConfigFinishPacket;
                        let mut buf    = EncodeBuf::new_len_prefixed(packet.encode_prefixed_len());
                        // SAFETY: `buf` has enough room for `packet.encode_prefixed_len()` bytes.
                        unsafe { packet.encode_prefixed(&mut buf); }
                        // SAFETY: `packet.encode_prefixed()` is required to fill the entire buffer.
                        self.bytes_to_write.extend(unsafe { buf.into_inner() });
                        self.outgoing_state.store(PacketState::Play, AtomicOrdering::SeqCst);
                    }
                    _ => {
                        #[cfg(not(debug_assertions))]
                        unreachable!("impossible switch state from {prev_state:?} to {switch_state:?}");
                        #[cfg(debug_assertions)]
                        {
                            let src = e.sent_by();
                            unreachable!("impossible switch state from {old_state:?} to {new_state:?} sent by {}:{}:{}", src.file(), src.line(), src.column());
                        }
                    }
                }
            }
            if let Some(b) = b {
                self.bytes_to_write.extend(b);
            }
        }

        if (e.is_kick()) {
            self.disconnecting.store(true, AtomicOrdering::Relaxed);
        }
    }

}


impl PacketSender for &mut PeerStreamWriter {

    fn with_before_switch<'l, T>(self, packet : T) -> Self
    where
        T : Into<S2CPackets<'l>>
    {
        self.handle_send_packet(&SendPacket::new(Entity::PLACEHOLDER).with_before_switch(packet));
        self
    }

    fn with<'l, T>(self, packet : T) -> Self
    where
        T : Into<S2CPackets<'l>>
    {
        self.handle_send_packet(&SendPacket::new(Entity::PLACEHOLDER).with(packet));
        self
    }

    fn with_switch_state(self, state : PacketState, skip_intermediate : bool) -> Self {
        self.handle_send_packet(&SendPacket::new(Entity::PLACEHOLDER).with_switch_state(state, skip_intermediate));
        self
    }

}
