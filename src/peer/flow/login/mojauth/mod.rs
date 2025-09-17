use crate::peer::PeerOptions;
use crate::game::player::{
    login::PlayerRequestLoginEvent,
    data::PlayerBundle
};
use crate::ecs::ParallelEventWriter;
use pipeworkmc_data::{
    character::NextCharacterId,
    profile::AccountProfile
};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    system::{
        ParallelCommands,
        Query,
        Res
    }
};
use bevy_tasks::{ Task, futures };


mod uri;
pub(super) use uri::*;


#[derive(Component)]
#[component(storage = "SparseSet")]
pub(in crate::peer) struct MojauthTask {
    pub(super) task        : Task<surf::Result<AccountProfile>>,
    pub(super) invalidated : bool
}


pub(in crate::peer) fn poll_mojauth_tasks(
        pcmds    : ParallelCommands,
    mut q_peers  : Query<(Entity, &mut MojauthTask)>,
        ew_login : ParallelEventWriter<PlayerRequestLoginEvent>,
        r_chid   : Res<NextCharacterId>
) {
    q_peers.par_iter_mut().for_each(|(entity, mut mojauth,)| {
        if (! mojauth.invalidated)
            && let Some(response) = futures::check_ready(&mut mojauth.task)
        {
            mojauth.invalidated = true;

            match (response) {
                Ok(profile) => {
                    ew_login.write(PlayerRequestLoginEvent::new(
                        entity, profile.uuid, profile.username.clone()
                    ));
                    pcmds.command_scope(|mut cmds| {
                        cmds.entity(entity)
                            .remove::<MojauthTask>()
                            .insert((
                                profile,
                                r_chid.next(),
                                PlayerBundle::default()
                            ));
                    });
                },
                Err(err) => panic!("{err:?}") // TODO: Error handler.
            };
        }
    });
}


pub(in crate::peer) fn is_mojauth_enabled(
    r_options : Res<PeerOptions>
) -> bool { r_options.mojauth_enabled }
