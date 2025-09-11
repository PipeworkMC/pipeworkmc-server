use crate::conn::peer::ConnPeerSender;
use crate::game::player::login::{
    PlayerRequestLoginEvent,
    PlayerApproveLoginEvent
};
use crate::ecs::ParallelEventWriter;
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
    mut q_peers    : Query<(Entity, &mut ConnPeerSender, &AccountProfile)>,
    mut er_request : EventReader<PlayerRequestLoginEvent>,
        ew_approve : ParallelEventWriter<PlayerApproveLoginEvent>
) {
    'request_loop : for e in er_request.read() {
        for (entity, mut sender, profile,) in &mut q_peers {
            if (e.peer() != entity && e.uuid() == profile.uuid) {
                sender.kick_duplicate_login();
                continue 'request_loop;
            }
        }
        ew_approve.write(PlayerApproveLoginEvent::from(e));
    }
}

fn reject_new_duplicates(
    mut q_peers    : Query<(Entity, &mut ConnPeerSender, &AccountProfile)>,
    mut er_request : EventReader<PlayerRequestLoginEvent>,
        ew_approve : ParallelEventWriter<PlayerApproveLoginEvent>
) {
    for e in er_request.read() {
        if (q_peers.iter().any(|(entity, _, profile,)| e.peer() != entity && e.uuid() == profile.uuid)) {
            if let Ok((_, mut sender, _,)) = q_peers.get_mut(e.peer()) {
                sender.kick_name_taken();
            }
        } else {
            ew_approve.write(PlayerApproveLoginEvent::from(e));
        }
    }
}
