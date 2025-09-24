use crate::peer::{
    PeerAddress,
    state::PeerState,
    writer::PacketSender,
    event::SendPacket
};
use crate::ecs::ParallelEventWriter;
use std::time::Instant;
use bevy_ecs::{
    entity::Entity,
    query::With,
    system::Query
};


pub(in crate::peer) fn timeout_peers(
    mut q_peers   : Query<(Entity, &PeerState,), (With<PeerAddress>,)>,
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
