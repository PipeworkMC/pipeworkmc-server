use super::PeerLoginFlow;
use crate::peer::{
    PeerAddress,
    PeerOptions
};
use crate::game::player::{
    login::PlayerRequestLoginEvent,
    data::PlayerBundle
};
use crate::ecs::ParallelEventWriter;
use pipeworkmc_data::character::NextCharacterId;
use bevy_ecs::{
    entity::Entity,
    query::With,
    system::{
        ParallelCommands,
        Query,
        Res
    }
};
use bevy_tasks::futures;


mod uri;
pub(super) use uri::*;


pub(in crate::peer) fn poll_mojauth_tasks(
        pcmds    : ParallelCommands,
    mut q_peers  : Query<(Entity, &mut PeerLoginFlow,), (With<PeerAddress>,)>,
        ew_login : ParallelEventWriter<PlayerRequestLoginEvent>,
        r_chid   : Res<NextCharacterId>
) {
    q_peers.par_iter_mut().for_each(|(entity, mut flow,)| {
        if let PeerLoginFlow::Mojauth { task } = &mut*flow
            && let Some(response) = futures::check_ready(task)
        {
            match (response) {
                Ok(profile) => {
                    ew_login.write(PlayerRequestLoginEvent::new(
                        entity, profile.uuid, profile.username.clone()
                    ));
                    pcmds.command_scope(|mut cmds| {
                        cmds.entity(entity)
                            .insert((
                                profile,
                                r_chid.next(),
                                PlayerBundle::default()
                            ));
                    });
                    *flow = PeerLoginFlow::Approval;
                },
                Err(err) => panic!("{err:?}") // TODO: Error handler.
            };
        }
    });
}


pub(in crate::peer) fn is_mojauth_enabled(
    r_options : Res<PeerOptions>
) -> bool { r_options.mojauth_enabled }
