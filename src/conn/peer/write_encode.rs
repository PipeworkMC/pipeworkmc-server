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
use crate::util::{
    ext::VecDequeExt,
    redacted::Redacted
};
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

#[derive(Component, Default)]
pub(in crate::conn) struct ConnPeerOutgoing {
    queue : VecDeque<u8>
}


pub(in crate::conn) fn encode_conn_peer_outgoing(
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

pub(in crate::conn) fn write_conn_peer_outgoing(
    mut q_peers : Query<(&mut ConnPeerWriter, &mut ConnPeerOutgoing,)>
) {
    q_peers.par_iter_mut().for_each(|(mut writer, mut outgoing,)| {
        let (slice0, slice1,) = outgoing.queue.as_slices();
        let mut outgoing_slice = (
            if (! slice0.is_empty()) { slice0 }
            else if (! slice1.is_empty()) { slice1 }
            else { return; }
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
                outgoing.queue.pop_many_front(count);
            },
            Err(err) if (err.kind() == io::ErrorKind::WouldBlock) => { },
            Err(err) => panic!("{err}") // TODO: Error handler.
        }
        // TODO: Kick if peer packet queue builds up too much.
    });
}
