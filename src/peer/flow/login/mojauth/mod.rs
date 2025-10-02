use super::PeerLoginFlow;
use crate::peer::{
    Peer,
    PeerOptions,
    PacketSender,
    SendPacket
};
use crate::game::{
    login::PlayerLoginRequest,
    character::{
        player::PlayerCharacterBundle,
        vis::VisibleCharacters
    }
};
use pipeworkmc_codec::meta::PacketState;
use pipeworkmc_packet::s2c::login::finish::S2CLoginFinishPacket;
use bevy_callback::Callback;
use bevy_ecs::{
    entity::Entity,
    message::MessageWriter,
    query::With,
    system::{
        Commands,
        Query,
        Res
    }
};
use bevy_tasks::futures;


mod uri;
pub(super) use uri::*;


/// Polls running mojauth tasks, requesting login approval once completed successfully.
pub(in crate::peer) fn poll_mojauth_tasks(
    mut cmds      : Commands,
    mut q_peers   : Query<(Entity, &mut PeerLoginFlow,), (With<Peer>,)>,
    mut mw_packet : MessageWriter<SendPacket>,
    mut c_login   : Callback<PlayerLoginRequest>,
) {
    for (entity, mut flow,) in &mut q_peers {
        if let PeerLoginFlow::Mojauth { task } = &mut*flow
            && let Some(response) = futures::check_ready(task)
        {
            match (response) {
                Ok(profile) => {
                    let approval = c_login.request(PlayerLoginRequest {
                        peer     : entity,
                        uuid     : profile.uuid,
                        username : profile.username.clone()
                    });
                    match (approval) {
                        Ok(()) => {
                            mw_packet.write(SendPacket::new(entity)
                                .with_before_switch(S2CLoginFinishPacket {
                                    profile : profile.clone()
                                })
                                .with_switch_state(PacketState::Config, true)
                            );
                            cmds.entity(entity).insert((
                                profile,
                                PlayerCharacterBundle::default(),
                                VisibleCharacters::new(entity)
                            ));
                            *flow = PeerLoginFlow::Acknowledge;
                        },
                        Err(reason) => {
                            mw_packet.write(SendPacket::new(entity).kick(&reason));
                            *flow = PeerLoginFlow::Done;
                        }
                    }
                },
                Err(err) => panic!("{err:?}") // TODO: Error handler.
            };
        }
    }
}


/// Returns `true` if mojauth is enabled.
pub(in crate::peer) fn is_mojauth_enabled(
    r_options : Res<PeerOptions>
) -> bool { r_options.mojauth_enabled }
