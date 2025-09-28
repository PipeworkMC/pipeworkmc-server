use crate::peer::{
    PacketSender,
    SendPacket
};
use crate::game::login::{
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


/// A [`Plugin`] which automatically handles duplicate logins on the server.
#[derive(Default)]
pub struct AutoApproveLoginsPlugin {
    /// How to handle duplicate logins on the server.
    pub duplicate_strategy : DuplicateStrategy
}

/// How to handle duplicate logins on the server.
#[derive(Default)]
pub enum DuplicateStrategy {
    /// Allow duplicate UUIDs.
    Allow,
    /// When a new player with the same UUID as an existing player joins, kick the older connection.
    #[default]
    KickOld,
    /// When a new player with the same UUID as an existing player joins, kick the newer connection.
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


/// Unconditionally approves login requests.
fn allow_duplicates(
    mut er_request : EventReader<PlayerRequestLoginEvent>,
        ew_approve : ParallelEventWriter<PlayerApproveLoginEvent>
) {
    er_request.par_read().for_each(|e| {
        ew_approve.write(PlayerApproveLoginEvent::from(e));
    });
}

/// Approves login requests, kicking any existing logins with the same UUID.
fn kick_old_duplicates(
        q_peers    : Query<(Entity, &AccountProfile)>,
    mut er_request : EventReader<PlayerRequestLoginEvent>,
        ew_packet  : ParallelEventWriter<SendPacket>,
        ew_approve : ParallelEventWriter<PlayerApproveLoginEvent>
) {
    er_request.par_read().for_each(|e| {
        for (entity, profile,) in &q_peers {
            if (e.peer != entity && e.uuid != profile.uuid) {
                ew_packet.write(SendPacket::new(entity).kick_duplicate_login());
                return;
            }
        }
        ew_approve.write(PlayerApproveLoginEvent::from(e));
    });
}

/// Approves login requests, as long as no existing logins have the same UUID.
fn reject_new_duplicates(
        q_peers    : Query<(Entity, &AccountProfile)>,
    mut er_request : EventReader<PlayerRequestLoginEvent>,
        ew_packet  : ParallelEventWriter<SendPacket>,
        ew_approve : ParallelEventWriter<PlayerApproveLoginEvent>
) {
    er_request.par_read().for_each(|e| {
        if (q_peers.iter().any(|(entity, profile,)| e.peer != entity && e.uuid == profile.uuid)) {
            ew_packet.write(SendPacket::new(e.peer).kick_name_taken());
        } else {
            ew_approve.write(PlayerApproveLoginEvent::from(e));
        }
    });
}
