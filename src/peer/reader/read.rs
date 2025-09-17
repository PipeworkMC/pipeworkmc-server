use crate::peer::{
    reader::PeerStreamReader,
    writer::PacketSender,
    state::PeerState,
    event::SendPacket
};
use crate::ecs::ParallelEventWriter;
use std::io::{ self, Read };
use bevy_ecs::{
    entity::Entity,
    system::Query
};


const READ_BYTES_PER_CYCLE : usize = 256;


pub(in crate::peer) fn read_peer_bytes(
    mut q_peers   : Query<(Entity, &mut PeerStreamReader, &PeerState,)>,
        ew_packet : ParallelEventWriter<SendPacket>
) {
    q_peers.par_iter_mut().for_each(|(entity, mut reader, state,)| {
        if (state.disconnecting()) { return; }
        let mut buf = [0u8; READ_BYTES_PER_CYCLE];
        match (reader.stream.read(&mut buf)) { // TODO: Ratelimit
            Ok(0) => { ew_packet.write(SendPacket::new(entity).kick_end_of_stream()); },
            Ok(count) => {
                let mut incoming_slice = &buf[0..count];

                let mut decrypted_buf = [0u8; READ_BYTES_PER_CYCLE + 1];
                if let Some(decrypter) = &mut reader.decrypter {
                    let count = unsafe { decrypter.as_mut().update_unchecked(incoming_slice, &mut decrypted_buf) }.unwrap(); // TODO: Error handler.
                    incoming_slice = &decrypted_buf[0..count];
                }

                reader.bytes_to_decode.extend(incoming_slice);
            },
            Err(err) if (err.kind() == io::ErrorKind::WouldBlock) => { },
            Err(err) => panic!("{err}") // TODO: Error handler.
        }
    });
}
