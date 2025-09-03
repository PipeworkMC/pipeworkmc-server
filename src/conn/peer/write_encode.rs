use crate::conn::protocol::{
    codec::encode::{ EncodeBuf, PrefixedPacketEncode },
    packet::{
        s2c::{
            S2CPackets,
            login::disconnect::S2CLoginDisconnectPacket,
            config::disconnect::S2CConfigDisconnectPacket,
            play::disconnect::S2CPlayDisconnectPacket
        },
        PacketMeta,
        AtomicPacketState, PacketState
    },
    value::{
        text::{ Text, TextComponent, TextContent },
        varint::VarIntType
    }
};
use crate::util::{
    ext::VecDequeExt,
    redacted::Redacted
};
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


const WRITE_BYTES_PER_CYCLE : usize = 256;


#[derive(Component)]
pub(in crate::conn) struct ConnPeerWriter {
    pub(in crate::conn) stream    : TcpStream,
    pub(in crate::conn) encrypter : Option<Redacted<Crypter>>
}
impl From<TcpStream> for ConnPeerWriter {
    #[inline(always)]
    fn from(stream : TcpStream) -> Self { Self { stream, encrypter : None } }
}

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
            (PacketState::Login,  S2CLoginDisconnectPacket::PREFIX,)
            | (PacketState::Config, S2CConfigDisconnectPacket::PREFIX,)
            => { self.disconnecting = true; },
            // TODO: Play
            _ => { }
        }

        let mut buf = EncodeBuf::new(packet.encode_prefixed_len());
        unsafe { packet.encode_prefixed(&mut buf); }
        // TODO: Compression
        let buf = buf.as_slice();
        self.queue.extend(<u32 as VarIntType>::encode(buf.len() as u32, &mut <u32 as VarIntType>::EncodeBuf::default()));
        self.queue.extend(buf);
    }


    pub fn kick<S>(&mut self, reason : S)
    where
        S : Into<Text>
    { match (self.outgoing_state.load(AtomicOrdering::SeqCst)) {
        PacketState::Handshake
        | PacketState::Status  => { self.disconnecting = true; },
        PacketState::Login     => { self.send(S2CLoginDisconnectPacket::from(reason)); },
        PacketState::Config    => { self.send(S2CConfigDisconnectPacket::from(reason)); },
        PacketState::Play      => { self.send(S2CPlayDisconnectPacket::from(reason)); }
    } }

    #[inline]
    pub fn kick_generic(&mut self) {
        self.kick(Text { components : Cow::Borrowed(&[ TextComponent { content : TextContent::Translate {
            key : Cow::Borrowed("multiplayer.disconnect.generic"), fallback : None, with : Cow::Borrowed(&[])
        }, ..TextComponent::EMPTY } ]) })
    }

    #[inline]
    pub fn kick_end_of_stream(&mut self) {
        self.kick(Text { components : Cow::Borrowed(&[ TextComponent { content : TextContent::Translate {
            key : Cow::Borrowed("disconnect.endOfStream"), fallback : None, with : Cow::Borrowed(&[])
        }, ..TextComponent::EMPTY } ]) })
    }

    #[inline]
    pub fn kick_packet_error<S>(&mut self, message : S)
    where
        S : Into<Text>
    {
        self.kick(Text { components : Cow::Borrowed(&[
            TextComponent { content : TextContent::Translate {
                key : Cow::Borrowed("disconnect.packetError"), fallback : None, with : Cow::Borrowed(&[])
            }, ..TextComponent::EMPTY },
            TextComponent { content : TextContent::Literal {
                text : Cow::Borrowed(": ")
            }, ..TextComponent::EMPTY }
        ]) } + message)
    }

    #[inline]
    pub fn kick_login_failed<S>(&mut self, message : S)
    where
        S : Into<Text>
    {
        self.kick(Text { components : Cow::Owned(vec![ TextComponent { content : TextContent::Translate {
            key : Cow::Borrowed("disconnect.loginFailedInfo"), fallback : None, with : Cow::Owned(vec![message.into()])
        }, ..TextComponent::EMPTY } ]) })
    }

    #[inline]
    pub fn kick_timeout(&mut self) {
        self.kick(Text { components : Cow::Borrowed(&[ TextComponent { content : TextContent::Translate {
            key : Cow::Borrowed("disconnect.timeout"), fallback : None, with : Cow::Borrowed(&[])
        }, ..TextComponent::EMPTY } ]) })
    }

}


pub(in crate::conn) fn write_conn_peer_outgoing(
        pcmds   : ParallelCommands,
    mut q_peers : Query<(Entity, &mut ConnPeerWriter, &mut ConnPeerSender,)>
) {
    q_peers.par_iter_mut().for_each(|(entity, mut writer, mut sender,)| {
        let (slice0, slice1,) = sender.queue.as_slices();
        let mut outgoing_slice = (
            if (! slice0.is_empty()) { slice0 }
            else if (! slice1.is_empty()) { slice1 }
            else { // All queued bytes have been sent.
                if (sender.disconnecting) {
                    pcmds.command_scope(|mut cmds| cmds.entity(entity).despawn());
                }
                return;
            }
        );
        if (outgoing_slice.len() > WRITE_BYTES_PER_CYCLE) {
            outgoing_slice = &outgoing_slice[0..WRITE_BYTES_PER_CYCLE];
        }

        let mut encrypted_buf = [0u8; WRITE_BYTES_PER_CYCLE + 1];
        if let Some(encrypter) = &mut writer.encrypter {
            let count = unsafe { encrypter.as_mut().update_unchecked(outgoing_slice, &mut encrypted_buf) }.unwrap(); // TODO: Error handler.
            outgoing_slice = &encrypted_buf[0..count];
        }

        match (writer.stream.write(outgoing_slice)) {
            Ok(count) => {
                sender.queue.pop_many_front(count);
            },
            Err(err) if (err.kind() == io::ErrorKind::WouldBlock) => { },
            Err(err) => panic!("{err}") // TODO: Error handler.
        }
        // TODO: Kick if peer packet queue builds up too much.
    });
}
