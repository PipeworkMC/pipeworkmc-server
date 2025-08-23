use crate::conn::{
    peer::{
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
    par_eventwriter::ParallelEventWriter
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


const READ_BYTES_PER_CYCLE : usize = 256;


#[derive(Component)]
pub(in super::super) struct ConnPeerReader {
    pub(in super::super) stream : TcpStream
}

#[derive(Component, Default)]
pub(in super::super) struct ConnPeerIncoming {
    queue : VecDeque<u8>
}
#[derive(Component, Default)]
pub(in super::super) struct ConnPeerDecoder {
    next_size : Option<usize>
}


pub(in super::super) fn read_conn_peer_incoming(
    mut q_peers : Query<(&mut ConnPeerReader, &mut ConnPeerIncoming,)>
) {
    q_peers.par_iter_mut().for_each(|(mut reader, mut incoming,)| {
        let mut buf = [0u8; READ_BYTES_PER_CYCLE];
        match (reader.stream.read(&mut buf)) {
            Ok(0) => { }, // TODO: Disconnected
            Ok(count) => {
                // TODO: Decrypt
                incoming.queue.extend(&buf[0..count]);
            },
            Err(err) if (err.kind() == io::ErrorKind::WouldBlock) => { },
            Err(err) => panic!("{err}") // TODO: Error handler.
        }
    });
}


pub(in super::super) fn decode_conn_peer_incoming(
    mut q_peers      : Query<(Entity, &mut ConnPeerIncoming, &mut ConnPeerDecoder, &ConnPeerState)>,
        ew_handshake : ParallelEventWriter<IncomingHandshakePacketEvent>,
        ew_status    : ParallelEventWriter<IncomingStatusPacketEvent>,
        ew_login     : ParallelEventWriter<IncomingLoginPacketEvent>
) {
    q_peers.par_iter_mut().for_each(|(peer, mut incoming, mut decoder, state)| {

        // Get or try to decode next packet size.
        let packet_size = decoder.next_size.get_or_maybe_insert_with(|| {
            match (<u32 as VarIntType>::decode(incoming.queue.iter().cloned())) {
                Ok((next_size, consumed,)) => {
                    incoming.queue.pop_many_front(consumed);
                    Some(next_size as usize)
                },
                Err(VarIntDecodeError::Incomplete) => None,
                Err(VarIntDecodeError::TooLong)    => panic!("{:?}", VarIntDecodeError::TooLong), // TODO: Error handler.
            }
        });

        // If enough bytes are present, decode a packet.
        if let Some(&mut packet_size) = packet_size
            && (incoming.queue.len() >= packet_size)
        {
            decoder.next_size = None;
            let     buf = unsafe { incoming.queue.pop_many_front_into_unchecked(packet_size) };
            let mut buf = DecodeBuf::from(&*buf);
            // TODO: Decompress
            match (state.incoming_state) {
                PacketState::Handshake => {
                    let packet = C2SHandshakePackets::decode_prefixed(&mut buf).unwrap(); // TODO: Error handler.
                    ew_handshake.write(IncomingHandshakePacketEvent::new(peer, packet));
                },
                PacketState::Status => {
                    let packet = C2SStatusPackets::decode_prefixed(&mut buf).unwrap(); // TODO: Error handler.
                    ew_status.write(IncomingStatusPacketEvent::new(peer, packet));
                },
                PacketState::Login  => {
                    let packet = C2SLoginPackets::decode_prefixed(&mut buf).unwrap(); // TODO: Error handler.
                    ew_login.write(IncomingLoginPacketEvent::new(peer, packet));
                },
                PacketState::Config => todo!(),
                PacketState::Play   => todo!()
            };
        }

    });
}
