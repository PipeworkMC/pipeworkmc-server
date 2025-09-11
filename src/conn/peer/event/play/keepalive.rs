use crate::conn::{
    peer::{
        ConnPeerSender,
        ConnPeerState
    },
    protocol::packet::s2c::{
        config::keep_alive::S2CConfigKeepAlivePacket,
        play::keep_alive::S2CPlayKeepAlivePacket
    }
};
use pipeworkmc_codec::meta::PacketState;
use bevy_ecs::system::Query;


pub(in crate::conn) fn handle_keepalive(
    mut peers : Query<(&mut ConnPeerSender, &ConnPeerState,)>
) {
    peers.par_iter_mut().for_each(|(mut sender, state,)| {
        match (state.outgoing_state()) {

            PacketState::Handshake
            | PacketState::Status
            | PacketState::Login => { },

            PacketState::Config => {
                sender.send(S2CConfigKeepAlivePacket { id : 0 });
            },

            PacketState::Play => {
                sender.send(S2CPlayKeepAlivePacket { id : 0 });
            }

        }
    });
}
