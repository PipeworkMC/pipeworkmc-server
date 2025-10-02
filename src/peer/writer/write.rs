use crate::peer::{
    Peer,
    writer::PeerStreamWriter,
    state::PeerState
};
use crate::game::login::PlayerLoggedOutMessage;
use crate::util::VecDequeExt;
use pipeworkmc_data::profile::AccountProfile;
use std::{
    io::{ self, Write },
    net::Shutdown
};
use bevy_ecs::{
    entity::Entity,
    query::With,
    system::{
        ParallelCommands,
        Query
    }
};
use bevy_parmessagewriter::ParallelMessageWriter;


const WRITE_BYTES_PER_CYCLE : usize = 256;


/// Encrypts and writes bytes to the stream.
pub(in crate::peer) fn write_peer_bytes(
        pcmds     : ParallelCommands,
    mut q_peers   : Query<(Entity, &mut PeerStreamWriter, &PeerState, Option<&AccountProfile>), (With<Peer>,)>,
        mw_logout : ParallelMessageWriter<PlayerLoggedOutMessage>
) {
    q_peers.par_iter_mut().for_each(|(entity, mut writer, state, profile,)| {
        let writer = &mut*writer;

        let (slice0, slice1,) = writer.bytes_to_write.as_slices();
        let mut outgoing_slice = (
            if (! slice0.is_empty()) { slice0 }
            else if (! slice1.is_empty()) { slice1 }
            else {
                if (state.disconnecting()) {
                    // All queued bytes have been sent. Shut down the connection.
                    _ = writer.stream.shutdown(Shutdown::Both);
                    pcmds.command_scope(|mut cmds| cmds.entity(entity).despawn());
                    if let Some(profile) = profile {
                        mw_logout.write(PlayerLoggedOutMessage {
                            peer     : entity,
                            uuid     : profile.uuid,
                            username : profile.username.clone()
                        });
                    }
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
            // SAFETY: `outgoing_slice` is never larger than `WRITE_BYTES_PER_CYCLE`.
            let count = unsafe { encrypter.as_mut().update_unchecked(outgoing_slice, &mut encrypted_buf) }.unwrap(); // TODO: Error handler.
            outgoing_slice = &encrypted_buf[0..count];
        }

        match (writer.stream.write(outgoing_slice)) {
            // Some bytes were sent. Remove them from the queue.
            Ok(count) => { writer.bytes_to_write.pop_many_front(count); },
            // No bytes were sent. Try again later.
            Err(err) if (err.kind() == io::ErrorKind::WouldBlock) => { },
            // Some other error occured.
            Err(err) => panic!("{err}") // TODO: Error handler.
        }
    });
}
