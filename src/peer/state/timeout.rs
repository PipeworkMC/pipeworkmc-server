use crate::peer::{
    state::PeerState,
    writer::PacketSender,
    event::SendPacket
};
use crate::ecs::ParallelEventWriter;
use std::time::Instant;
use bevy_ecs::{
    entity::Entity,
    system::Query
};


pub(in crate::peer) fn timeout_peers(
    mut q_peers   : Query<(Entity, &PeerState,)>,
        ew_packet : ParallelEventWriter<SendPacket>
) {
    q_peers.par_iter_mut().for_each(|(entity, state,)| {
        if (Instant::now() > state.expires) {
            ew_packet.write(SendPacket::new(entity).kick_timeout());
        }
    });
}
