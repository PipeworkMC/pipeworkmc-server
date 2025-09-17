use pipeworkmc_codec::{
    encode::{
        EncodeBuf,
        PrefixedPacketEncode
    },
    meta::{
        PacketMeta,
        AtomicPacketState,
        PacketState
    }
};
use pipeworkmc_data::{
    redacted::Redacted,
    text::{ Text, TextComponent, TextContent },
    varint::VarIntType
};
use pipeworkmc_packet::s2c::{
    S2CPackets,
    login::disconnect::S2CLoginDisconnectPacket,
    config::disconnect::S2CConfigDisconnectPacket,
    play::disconnect::S2CPlayDisconnectPacket
};
use crate::util::VecDequeExt;
use std::{
    borrow::Cow,
    collections::VecDeque,
    io::{ self, Write },
    net::TcpStream,
    sync::{ Arc, atomic::Ordering as AtomicOrdering }
};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    system::{ ParallelCommands, Query }
};
use openssl::symm::Crypter;


#[derive(Component)]
pub struct ConnPeerSender {
    queue          : VecDeque<u8>,
    outgoing_state : Arc<AtomicPacketState>,
    disconnecting  : bool
}
impl From<Arc<AtomicPacketState>> for ConnPeerSender {
    #[inline]
    fn from(outgoing_state : Arc<AtomicPacketState>) -> Self { Self {
        queue          : VecDeque::new(),
        outgoing_state,
        disconnecting  : false
    } }
}


impl ConnPeerSender {

    #[inline(always)]
    pub fn is_disconnecting(&self) -> bool { self.disconnecting }

    pub fn send<'l, P>(&mut self, packet : P)
    where
        P : Into<S2CPackets<'l>>
    { // TODO: Switch state if needed.
        if (self.disconnecting) { return; }
        let packet = packet.into();

        match (packet.meta()) {
              (PacketState::Login,    S2CLoginDisconnectPacket  ::PREFIX,)
            | (PacketState::Config, S2CConfigDisconnectPacket ::PREFIX,)
            | (PacketState::Play,   S2CPlayDisconnectPacket   ::PREFIX,)
            => { self.disconnecting = true; },
            _ => { }
        }

        let mut buf = EncodeBuf::new(packet.encode_prefixed_len());
        unsafe { packet.encode_prefixed(&mut buf); }
        // TODO: Compression
        let buf = buf.as_slice();
        self.queue.extend(<u32 as VarIntType>::encode(buf.len() as u32, &mut <u32 as VarIntType>::EncodeBuf::default()));
        self.queue.extend(buf);
    }

}
