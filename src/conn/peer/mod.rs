use crate::conn::{
    peer::event::{
        IncomingHandshakePacketEvent,
        IncomingStatusPacketEvent
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
                status::C2SStatusPackets
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
use core::{
    net::SocketAddr,
    time::Duration
};
use std::{
    collections::VecDeque,
    io::{ self, Read },
    net::TcpStream,
    time::Instant
};
use bevy_ecs::{
    bundle::Bundle,
    component::Component,
    entity::Entity,
    system::Query
};


pub mod event;


const READ_BYTES_PER_CYCLE : usize = 256;


#[derive(Bundle)]
pub(super) struct ConnPeerBundle {
    pub(super) peer     : ConnPeer,
    pub(super) reader   : ConnPeerReader,
    pub(super) incoming : ConnPeerIncoming,
    pub(super) decoder  : ConnPeerDecoder,
    // pub(super) writer   : ConnPeerWriter,
    pub(super) state    : ConnPeerState
}


#[derive(Component)]
pub struct ConnPeer {
    #[expect(dead_code)]
    addr : SocketAddr
}

impl From<SocketAddr> for ConnPeer {
    fn from(addr : SocketAddr) -> Self {
        Self { addr }
    }
}

#[derive(Component)]
pub struct ConnPeerState { // TODO: Timeout handshake/status/login
    incoming_state : PacketState,
    outgoing_state : PacketState,
    expires        : Option<Instant>
}
impl ConnPeerState {

    pub fn handshake() -> Self { Self {
        incoming_state : PacketState::Handshake,
        outgoing_state : PacketState::Handshake,
        expires        : Some(Instant::now() + Duration::from_millis(1000))
    } }

    pub fn switch_to_status(&mut self) {
        self.incoming_state = PacketState::Status;
        self.outgoing_state = PacketState::Status;
        self.expires        = Some(Instant::now() + Duration::from_millis(1000));
    }

    pub fn switch_to_login(&mut self) {
        self.incoming_state = PacketState::Login;
        self.outgoing_state = PacketState::Login;
        self.expires        = Some(Instant::now() + Duration::from_millis(1000));
    }

}


#[derive(Component)]
pub(super) struct ConnPeerReader {
    pub(super) stream : TcpStream
}

#[derive(Component, Default)]
pub(super) struct ConnPeerIncoming {
    queue : VecDeque<u8>
}
#[derive(Component, Default)]
pub(super) struct ConnPeerDecoder {
    next_size : Option<usize>
}


// #[derive(Component)]
// pub(super) struct ConnPeerWriter {
//     pub(super) stream : TcpStream
// }


pub(super) fn read_conn_peer_incoming(
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

pub(super) fn decode_conn_peer_incoming(
    mut q_peers      : Query<(Entity, &mut ConnPeerIncoming, &mut ConnPeerDecoder, &ConnPeerState)>,
        ew_handshake : ParallelEventWriter<IncomingHandshakePacketEvent>,
        ew_status    : ParallelEventWriter<IncomingStatusPacketEvent>
) {
    q_peers.par_iter_mut().for_each(|(peer, mut incoming, mut decoder, state)| {

        // Get or try to decode next packet size.
        let packet_size = decoder.next_size.get_or_maybe_insert_with(|| {
            match (<u32 as VarIntType>::decode(incoming.queue.iter().cloned())) {
                Ok((next_size, consumed,)) => {
                    incoming.queue.pop_many_front(consumed);
                    let next_size = *next_size as usize;
                    Some(next_size)
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
                    ew_handshake.write(IncomingHandshakePacketEvent { peer, packet, timestamp : Instant::now()});
                },
                PacketState::Status => {
                    let packet = C2SStatusPackets::decode_prefixed(&mut buf).unwrap(); // TODO: Error handler.
                    ew_status.write(IncomingStatusPacketEvent { peer, packet, timestamp : Instant::now()});
                },
                PacketState::Login  => todo!(),
                PacketState::Config => todo!(),
                PacketState::Play   => todo!()
            };
        }

    });
}


// pub(super) fn write_conn_peer_outgoing(
//     mut q_peers : Query<(&mut ConnPeerWriter, &mut ConnPeerOutgoing,)>
// ) {
//     q_peers.par_iter_mut().for_each(|(mut writer, mut outgoing,)| {
//         let (slice0, slice1,) = outgoing.0.as_slices();
//         let outgoing_slice = (
//             if (! slice0.is_empty()) { slice0 }
//             else if (! slice1.is_empty()) { slice1 }
//             else { return; }
//         );
//         match (writer.0.write(outgoing_slice)) {
//             Ok(count) => {
//                 for _ in 0..count {
//                     outgoing.0.pop_front();
//                 }
//             },
//             Err(err) if (err.kind() == io::ErrorKind::WouldBlock) => { },
//             Err(err) => panic!("{err}") // TODO: Error handler.
//         }
//         // TODO: Kick if peer refuses packets for too long.
//     });
// }
