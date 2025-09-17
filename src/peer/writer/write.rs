use crate::peer::{
    writer::PeerStreamWriter,
    state::PeerState
};
use crate::util::VecDequeExt;
use std::io::{ self, Write };
use bevy_ecs::{
    entity::Entity,
    system::{
        ParallelCommands,
        Query
    }
};


const WRITE_BYTES_PER_CYCLE : usize = 256;


pub(in crate::peer) fn write_peer_bytes(
        pcmds   : ParallelCommands,
    mut q_peers : Query<(Entity, &mut PeerStreamWriter, &PeerState)>
) {
    q_peers.par_iter_mut().for_each(|(entity, mut writer, state,)| {
        let writer = &mut*writer;

        let (slice0, slice1,) = writer.bytes_to_write.as_slices();
        let mut outgoing_slice = (
            if (! slice0.is_empty()) { slice0 }
            else if (! slice1.is_empty()) { slice1 }
            else { // All queued bytes have been sent.
                if (state.disconnecting()) {
                    pcmds.command_scope(|mut cmds| cmds.entity(entity).despawn());
                }
                return;
            }
        );
        if (outgoing_slice.len() > WRITE_BYTES_PER_CYCLE) {
            outgoing_slice = &outgoing_slice[0..WRITE_BYTES_PER_CYCLE];
        }

        // TODO: Kick on writer.bytes_to_write too large (timed out?).

        let mut encrypted_buf = [0u8; WRITE_BYTES_PER_CYCLE + 1];
        if let Some(encrypter) = &mut writer.encrypter {
            let count = unsafe { encrypter.as_mut().update_unchecked(outgoing_slice, &mut encrypted_buf) }.unwrap(); // TODO: Error handler.
            outgoing_slice = &encrypted_buf[0..count];
        }

        match (writer.stream.write(outgoing_slice)) {
            Ok(count) => { writer.bytes_to_write.pop_many_front(count); },
            Err(err) if (err.kind() == io::ErrorKind::WouldBlock) => { },
            Err(err) => panic!("{err}") // TODO: Error handler.
        }
        // TODO: Kick if peer packet queue builds up too much.
    });
}
