use crate::peer::event::SendPacket;
use crate::game::player::login::{
    PlayerRequestLoginEvent,
    PlayerApproveLoginEvent
};
use crate::ecs::ParallelEventWriter;
use crate::peer::writer::PacketSender;
use pipeworkmc_data::profile::AccountProfile;
use bevy_app::{ App, Plugin, Update };
use bevy_ecs::{
    entity::Entity,
    event::EventReader,
    system::Query
};


#[derive(Default)]
pub struct AutoApproveLoginsPlugin {
    pub duplicate_strategy : DuplicateStrategy
}

#[derive(Default)]
pub enum DuplicateStrategy {
    Allow,
    #[default]
    KickOld,
    RejectNew
}


impl Plugin for AutoApproveLoginsPlugin {
    fn build(&self, app : &mut App) {
        match (self.duplicate_strategy) {

            DuplicateStrategy::Allow => { app.add_systems(Update, allow_duplicates); },

            DuplicateStrategy::KickOld => { app.add_systems(Update, kick_old_duplicates); },

            DuplicateStrategy::RejectNew => { app.add_systems(Update, reject_new_duplicates); }

        }
    }
}


fn allow_duplicates(
    mut er_request : EventReader<PlayerRequestLoginEvent>,
        ew_approve : ParallelEventWriter<PlayerApproveLoginEvent>
) {
    er_request.par_read().for_each(|e| {
        ew_approve.write(PlayerApproveLoginEvent::from(e));
    });
}

fn kick_old_duplicates(
        q_peers    : Query<(Entity, &AccountProfile)>,
    mut er_request : EventReader<PlayerRequestLoginEvent>,
        ew_packet  : ParallelEventWriter<SendPacket>,
        ew_approve : ParallelEventWriter<PlayerApproveLoginEvent>
) {
    er_request.par_read().for_each(|e| {
        for (entity, profile,) in &q_peers {
            if (e.peer() != entity && e.uuid() != profile.uuid) {
                ew_packet.write(SendPacket::new(entity).kick_duplicate_login());
                return;
            }
        }
        ew_approve.write(PlayerApproveLoginEvent::from(e));
    });
}

fn reject_new_duplicates(
        q_peers    : Query<(Entity, &AccountProfile)>,
    mut er_request : EventReader<PlayerRequestLoginEvent>,
        ew_packet  : ParallelEventWriter<SendPacket>,
        ew_approve : ParallelEventWriter<PlayerApproveLoginEvent>
) {
    er_request.par_read().for_each(|e| {
        if (q_peers.iter().any(|(entity, profile,)| e.peer() != entity && e.uuid() == profile.uuid)) {
            ew_packet.write(SendPacket::new(e.peer()).kick_name_taken());
        } else {
            ew_approve.write(PlayerApproveLoginEvent::from(e));
        }
    });
}
