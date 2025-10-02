use crate::peer::{
    Peer,
    reader::PeerStreamReader,
    writer::PacketSender,
    state::PeerState,
    message::SendPacket
};
use std::io::{ self, Read };
use bevy_ecs::{
    entity::Entity,
    query::With,
    system::Query
};
use bevy_parmessagewriter::ParallelMessageWriter;


const READ_BYTES_PER_CYCLE : usize = 256;


/// Reads bytes from the stream, decrypting and queueing them for decoding.
pub(in crate::peer) fn read_peer_bytes(
    mut q_peers   : Query<(Entity, &mut PeerStreamReader, &PeerState,), (With<Peer>,)>,
        mw_packet : ParallelMessageWriter<SendPacket>
) {
    q_peers.par_iter_mut().for_each(|(entity, mut reader, state,)| {
        if (state.disconnecting()) { return; }
        let mut buf = [0u8; READ_BYTES_PER_CYCLE];
        match (reader.stream.read(&mut buf)) { // TODO: Ratelimit
            // No bytes received. Connection closed by peer.
            Ok(0) => { mw_packet.write(SendPacket::new(entity).kick_end_of_stream()); },
            // Bytes received. Decrypt and queue them for decoding.
            Ok(count) => {
                let mut incoming_slice = &buf[0..count];

                let mut decrypted_buf = [0u8; READ_BYTES_PER_CYCLE + 1];
                if let Some(decrypter) = &mut reader.decrypter {
                    // SAFETY: `incoming_slice` is never larger than `WRITE_BYTES_PER_CYCLE`.
                    let count = unsafe { decrypter.as_mut().update_unchecked(incoming_slice, &mut decrypted_buf) }.unwrap(); // TODO: Error handler.
                    incoming_slice = &decrypted_buf[0..count];
                }

                reader.bytes_to_decode.extend(incoming_slice);
            },
            // No bytes received. Wait for more.
            Err(err) if (err.kind() == io::ErrorKind::WouldBlock) => { },
            // Some error occured.
            Err(err) => panic!("{err}") // TODO: Error handler.
        }
    });
}
