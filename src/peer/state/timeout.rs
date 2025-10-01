use crate::peer::{
    Peer,
    state::PeerState,
    writer::PacketSender,
    event::SendPacket
};
use std::time::Instant;
use bevy_ecs::{
    entity::Entity,
    query::With,
    system::Query
};
use bevy_pareventwriter::ParallelEventWriter;


pub(in crate::peer) fn timeout_peers(
    mut q_peers   : Query<(Entity, &PeerState,), (With<Peer>,)>,
        ew_packet : ParallelEventWriter<SendPacket>
) {
    q_peers.par_iter_mut().for_each(|(entity, state,)| {
        if (! state.disconnecting())
            && let Some(expires) = state.expires
            && (Instant::now() >= expires)
        {
            ew_packet.write(SendPacket::new(entity).kick_timeout());
        }
    });
}
