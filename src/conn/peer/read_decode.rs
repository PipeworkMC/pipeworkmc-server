use crate::conn::{
    peer::{
        ConnPeerSender,
        ConnPeerState,
        event::{
            handshake::IncomingHandshakePacketEvent,
            status::IncomingStatusPacketEvent,
            login::IncomingLoginPacketEvent
        }
    },
    protocol::{
        codec::decode::{
            PrefixedPacketDecode,
            DecodeBuf
        },
        packet::{
            PacketState,
            c2s::{
                handshake::C2SHandshakePackets,
                status::C2SStatusPackets,
                login::C2SLoginPackets
            }
        },
        value::varint::{
            VarIntType,
            VarIntDecodeError
        }
    }
};
use crate::util::{
    ext::{
        OptionExt,
        VecDequeExt
    },
    par_eventwriter::ParallelEventWriter,
    redacted::Redacted
};
use std::{
    collections::VecDeque,
    io::{ self, Read },
    net::TcpStream
};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    system::Query
};
use openssl::symm::Crypter;


const READ_BYTES_PER_CYCLE : usize = 256;


#[derive(Component)]
pub(in crate::conn) struct ConnPeerReader {
    pub(in crate::conn)       stream    : TcpStream,
    pub(in crate::conn::peer) decrypter : Option<Redacted<Crypter>>
}
impl From<TcpStream> for ConnPeerReader {
    #[inline(always)]
    fn from(stream : TcpStream) -> Self { Self { stream, decrypter : None } }
}

#[derive(Component, Default)]
pub(in crate::conn) struct ConnPeerIncoming {
    queue : VecDeque<u8>
}
#[derive(Component, Default)]
pub(in crate::conn) struct ConnPeerDecoder {
    next_size : Option<usize>
}


pub(in crate::conn) fn read_conn_peer_incoming(
    mut q_peers : Query<(&mut ConnPeerReader, &mut ConnPeerIncoming, &mut ConnPeerSender,)>
) {
    q_peers.par_iter_mut().for_each(|(mut reader, mut incoming, mut sender,)| {
        if (sender.is_disconnecting()) { return; }
        let mut buf = [0u8; READ_BYTES_PER_CYCLE];
        match (reader.stream.read(&mut buf)) { // TODO: Ratelimit
            Ok(0) => { sender.kick_end_of_stream(); },
            Ok(count) => {
                let mut incoming_slice = &buf[0..count];

                let mut decrypted_buf = [0u8; READ_BYTES_PER_CYCLE + 1];
                if let Some(decrypter) = &mut reader.decrypter {
                    let count = unsafe { decrypter.as_mut().update_unchecked(incoming_slice, &mut decrypted_buf) }.unwrap(); // TODO: Error handler.
                    incoming_slice = &decrypted_buf[0..count];
                }

                incoming.queue.extend(incoming_slice);
            },
            Err(err) if (err.kind() == io::ErrorKind::WouldBlock) => { },
            Err(err) => panic!("{err}") // TODO: Error handler.
        }
    });
}


pub(in crate::conn) fn decode_conn_peer_incoming(
    mut q_peers      : Query<(Entity, &mut ConnPeerIncoming, &mut ConnPeerDecoder, &mut ConnPeerSender, &ConnPeerState)>,
        ew_handshake : ParallelEventWriter<IncomingHandshakePacketEvent>,
        ew_status    : ParallelEventWriter<IncomingStatusPacketEvent>,
        ew_login     : ParallelEventWriter<IncomingLoginPacketEvent>
) {
    q_peers.par_iter_mut().for_each(|(peer, mut incoming, mut decoder, mut sender, state)| {
        if (sender.is_disconnecting()) { return; }

        // Get or try to decode next packet size.
        let packet_size = decoder.next_size.get_or_maybe_insert_with(|| {
            match (<u32 as VarIntType>::decode(incoming.queue.iter().cloned())) {
                Ok((next_size, consumed,)) => {
                    incoming.queue.pop_many_front(consumed);
                    Some(next_size as usize)
                },
                Err(VarIntDecodeError::Incomplete(_)) => None,
                Err(VarIntDecodeError::TooLong)       => panic!("{:?}", VarIntDecodeError::TooLong), // TODO: Error handler.
            }
        });

        // TODO: Cap packet length.

        // If enough bytes are present, decode a packet.
        if let Some(&mut packet_size) = packet_size
            && (incoming.queue.len() >= packet_size)
        {
            decoder.next_size = None;
            let     buf = unsafe { incoming.queue.pop_many_front_into_unchecked(packet_size) };
            let mut buf = DecodeBuf::from(&*buf);
            // TODO: Decompress
            match (state.incoming_state) {
                PacketState::Handshake => { match (C2SHandshakePackets::decode_prefixed(&mut buf)) {
                    Ok(packet) => { ew_handshake.write(IncomingHandshakePacketEvent::new(peer, packet)); },
                    Err(err)   => { sender.kick_packet_error(format!("handshake {err}")); }
                } },
                PacketState::Status => { match (C2SStatusPackets::decode_prefixed(&mut buf)) {
                    Ok(packet) => { ew_status.write(IncomingStatusPacketEvent::new(peer, packet)); },
                    Err(err)   => { sender.kick_packet_error(format!("status {err}")); }
                } },
                PacketState::Login => { match (C2SLoginPackets::decode_prefixed(&mut buf)) {
                    Ok(packet) => { ew_login.write(IncomingLoginPacketEvent::new(peer, packet)); },
                    Err(err)   => { sender.kick_packet_error(format!("login {err}")); }
                } },
                PacketState::Config => todo!(),
                PacketState::Play   => todo!()
            };
        }

    });
}
