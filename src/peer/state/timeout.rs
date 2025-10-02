use crate::peer::{
    Peer,
    state::PeerState,
    writer::PacketSender,
    message::SendPacket
};
use std::time::Instant;
use bevy_ecs::{
    entity::Entity,
    query::With,
    system::Query
};
use bevy_parmessagewriter::ParallelMessageWriter;


pub(in crate::peer) fn timeout_peers(
    mut q_peers   : Query<(Entity, &PeerState,), (With<Peer>,)>,
        mw_packet : ParallelMessageWriter<SendPacket>
) {
    q_peers.par_iter_mut().for_each(|(entity, state,)| {
        if (! state.disconnecting())
            && let Some(expires) = state.expires
            && (Instant::now() >= expires)
        {
            mw_packet.write(SendPacket::new(entity).kick_timeout());
        }
    });
}
