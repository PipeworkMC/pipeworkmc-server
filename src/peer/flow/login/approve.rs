use crate::peer::{
    writer::PacketSender,
    event::SendPacket
};
use crate::game::player::login::{
    PlayerApproveLoginEvent
};
use crate::ecs::ParallelEventWriter;
use pipeworkmc_data::profile::AccountProfile;
use pipeworkmc_packet::s2c::login::finish::S2CLoginFinishPacket;
use bevy_ecs::{
    event::EventReader,
    system::Query
};


pub(in crate::peer) fn alert_approved_logins(
        q_peers   : Query<(&AccountProfile,)>,
    mut er_login  : EventReader<PlayerApproveLoginEvent>,
        ew_packet : ParallelEventWriter<SendPacket>
) {
    er_login.par_read().for_each(|e| {
        if let Ok((profile,)) = q_peers.get(e.entity()) {
            ew_packet.write(SendPacket::new(e.entity()).with(S2CLoginFinishPacket {
                profile : AccountProfile {
                    uuid     : profile.uuid,
                    username : profile.username.clone(),
                    skin     : None
                }
            }));
        }
    });
}
