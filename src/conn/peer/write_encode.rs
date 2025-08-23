use crate::conn::{
    peer::event::OutgoingPacketEvent,
    protocol::{
        codec::encode::{
            PrefixedPacketEncode,
            EncodeBuf
        },
        value::varint::VarIntType
    }
};
use crate::util::ext::VecDequeExt;
use std::{
    collections::VecDeque,
    io::{ self, Write },
    net::TcpStream
};
use bevy_ecs::{
    component::Component,
    event::EventReader,
    system::Query
};


#[derive(Component)]
pub(in super::super) struct ConnPeerWriter {
    pub(in super::super) stream : TcpStream
}

#[derive(Component, Default)]
pub(in super::super) struct ConnPeerOutgoing {
    queue : VecDeque<u8>
}


pub(in super::super) fn encode_conn_peer_outgoing(
    mut q_peers   : Query<(&mut ConnPeerOutgoing,)>,
    mut er_packet : EventReader<OutgoingPacketEvent>
) {
    for event in er_packet.read() {
        if let Ok((mut outgoing,)) = q_peers.get_mut(event.peer()) {
            let     packet = event.packet();
            let mut buf    = EncodeBuf::new(packet.encode_prefixed_len());
            unsafe { packet.encode_prefixed(&mut buf); }
            // TODO: Compression

            let buf = buf.as_slice();
            outgoing.queue.extend(<u32 as VarIntType>::encode(buf.len() as u32, &mut <u32 as VarIntType>::EncodeBuf::default()));
            outgoing.queue.extend(buf);
        }
    }
}

pub(in super::super) fn write_conn_peer_outgoing(
    mut q_peers : Query<(&mut ConnPeerWriter, &mut ConnPeerOutgoing,)>
) {
    q_peers.par_iter_mut().for_each(|(mut writer, mut outgoing,)| {
        let (slice0, slice1,) = outgoing.queue.as_slices();
        let outgoing_slice = (
            if (! slice0.is_empty()) { slice0 }
            else if (! slice1.is_empty()) { slice1 }
            else { return; }
        );
        // TODO: Encryption
        match (writer.stream.write(outgoing_slice)) {
            Ok(count) => {
                outgoing.queue.pop_many_front(count);
            },
            Err(err) if (err.kind() == io::ErrorKind::WouldBlock) => { },
            Err(err) => panic!("{err}") // TODO: Error handler.
        }
        // TODO: Kick if peer packet queue builds up too much.
    });
}
