use crate::conn::{
    ConnOptions,
    peer::{
        event::login::IncomingLoginPacketEvent,
        ConnPeerReader,
        ConnPeerWriter,
        ConnPeerSender,
        ConnPeerState
    }
};
use crate::game::player::{
    login::{
        PlayerRequestLoginEvent,
        PlayerApproveLoginEvent,
        PlayerLoggedInEvent
    },
    data::{
        dimension::Dimension,
        IsHardcore,
        ViewDistance,
        ReducedDebugInfo,
        NoRespawnScreen
    }
};
use pipeworkmc_codec::Protocol;
use pipeworkmc_data::{
    bounded_string::BoundedString,
    cat_variant::CatVariant,
    channel_data::ChannelData,
    character::{
        CharacterId,
        NextCharacterId
    },
    chicken_variant::ChickenVariant,
    cow_variant::CowVariant,
    damage_type::DamageType,
    frog_variant::FrogVariant,
    game_mode::GameMode,
    known_pack::KnownPack,
    painting_variant::PaintingVariant,
    pig_variant::PigVariant,
    profile::AccountProfile,
    redacted::Redacted,
    registry_entry::RegistryEntry,
    uuid::Uuid,
    wolf_variant::WolfVariant,
    wolf_sound_variant::WolfSoundVariant,
    worldgen::biome::WorldgenBiome
};
use pipeworkmc_packet::{
    c2s::login::{
        C2SLoginPackets,
        start::C2SLoginStartPacket,
        encrypt_response::C2SLoginEncryptResponsePacket,
        finish_acknowledged::C2SLoginFinishAcknowledgedPacket
    },
    s2c::{
        login::{
            encrypt_request::S2CLoginEncryptRequestPacket,
            finish::S2CLoginFinishPacket
        },
        config::{
            custom_payload::S2CConfigCustomPayloadPacket,
            finish::S2CConfigFinishPacket,
            registry_data::S2CConfigRegistryDataPacket,
            known_packs::S2CConfigKnownPacksPacket
        },
        play::login::S2CPlayLoginPacket
    }
};
use core::{
    hint::unreachable_unchecked,
    ptr
};
use std::borrow::Cow;
use bevy_ecs::{
    component::Component,
    entity::Entity,
    event::EventReader,
    query::Has,
    system::{ Commands, Query, Res, ResMut }
};
use bevy_tasks::{ IoTaskPool, Task, futures };
use openssl::{
    pkey::Private,
    rsa::{ Padding, Rsa },
    sha::Sha1,
    symm::{ Cipher, Crypter, Mode as CrypterMode }
};
use ethnum::I256 as i256;
use rand::RngCore;


pub(in crate::conn) fn approve_logins(
    mut q_peers    : Query<(
        &mut ConnPeerSender,
        &mut ConnPeerState,
        &mut ConnPeerLoginFlow,
        &AccountProfile,
    ),>,
    mut er_approve : EventReader<PlayerApproveLoginEvent>
) {
    for e in er_approve.read() {
        if let Ok((mut sender, mut state, mut login_flow, profile,)) = q_peers.get_mut(e.peer()) {
            if (sender.is_disconnecting()) { continue; }
            login_flow.approved = true;
            sender.send(S2CLoginFinishPacket { profile : profile.clone() });
            unsafe { state.login_finish(); }
        }
    }
}


#[expect(clippy::type_complexity)]
pub(in crate::conn) fn finalise_logins(
    mut cmds      : Commands,
    mut q_peers   : Query<(
        Entity,
        &mut ConnPeerSender,
        &mut ConnPeerState,
        &ConnPeerLoginFlow,
        &CharacterId,
        &AccountProfile,
        Has<IsHardcore>,
        &Dimension,
        &ViewDistance,
        Has<ReducedDebugInfo>,
        Has<NoRespawnScreen>,
        &GameMode
    )>,
    mut er_login  : EventReader<IncomingLoginPacketEvent>,
        r_options : Res<ConnOptions>
) {
    for event in er_login.read() {
        if let C2SLoginPackets::FinishAcknowledged(_) = event.packet()
            && let Ok((
                entity,
                mut sender,
                mut state,
                login_flow,
                chid,
                profile,
                is_hardcore,
                dimension,
                view_dist,
                reduced_debug_info,
                no_respawn_screen,
                game_mode,
            )) = q_peers.get_mut(event.peer())
        {
            if (sender.is_disconnecting()) { continue; }
            if (! login_flow.approved) {
                sender.kick_login_failed("Login not yet approved");
                continue;
            };

            unsafe { state.login_finish_acknowledged(); }

            let mut ecmds = cmds.entity(entity);
            ecmds.remove::<ConnPeerLoginFlow>();

            // TODO: Generate and use vanilla registries.
            sender.send(S2CConfigCustomPayloadPacket { data : ChannelData::Brand {
                brand : Cow::Borrowed(&r_options.server_brand)
            } });

            sender.send(S2CConfigKnownPacksPacket { known_packs : Cow::Borrowed(&[ KnownPack {
                namespace : Cow::Borrowed("minecraft"),
                id        : Cow::Borrowed("core"),
                version   : Cow::Borrowed(Protocol::LATEST.latest_name())
            } ]) });

            sender.send(S2CConfigRegistryDataPacket::from(CatVariant::VANILLA_REGISTRY_ENTRIES)); // TODO: Make these customisable.
            sender.send(S2CConfigRegistryDataPacket::from(ChickenVariant::VANILLA_REGISTRY_ENTRIES));
            sender.send(S2CConfigRegistryDataPacket::from(CowVariant::VANILLA_REGISTRY_ENTRIES));
            sender.send(S2CConfigRegistryDataPacket::from(DamageType::VANILLA_REGISTRY_ENTRIES));
            sender.send(S2CConfigRegistryDataPacket::from(FrogVariant::VANILLA_REGISTRY_ENTRIES));
            sender.send(S2CConfigRegistryDataPacket::from(PaintingVariant::VANILLA_REGISTRY_ENTRIES));
            sender.send(S2CConfigRegistryDataPacket::from(PigVariant::VANILLA_REGISTRY_ENTRIES));
            sender.send(S2CConfigRegistryDataPacket::from(WolfVariant::VANILLA_REGISTRY_ENTRIES));
            sender.send(S2CConfigRegistryDataPacket::from(WolfSoundVariant::VANILLA_REGISTRY_ENTRIES));
            sender.send(S2CConfigRegistryDataPacket::from(WorldgenBiome::VANILLA_REGISTRY_ENTRIES));

            sender.send(S2CConfigRegistryDataPacket::from(&[
                RegistryEntry { id : dimension.id.clone(), data : &dimension.dim_type }
            ]));

            sender.send(S2CConfigFinishPacket);
            unsafe { state.config_finish(); }
            sender.send(S2CPlayLoginPacket { // TODO: Finish logging in.
                eid                  : *chid,
                hardcore             : is_hardcore,
                all_dim_ids          : Cow::Owned(vec![dimension.id.clone()]),
                max_players          : 0,
                view_dist            : view_dist.as_u8() as u32,
                sim_dist             : 32,
                reduced_debug_info,
                respawn_screen       : ! no_respawn_screen,
                limited_crafting     : true,
                dim_type             : 0,
                dim_id               : dimension.id.clone(),
                hashed_seed          : dimension.hashed_seed,
                game_mode            : *game_mode,
                prev_game_mode       : None,
                is_debug_world       : dimension.is_debug,
                is_flat_world        : dimension.is_flat,
                death_location       : None,
                portal_cooldown      : 0,
                sea_level            : dimension.sea_level,
                enforces_secure_chat : false
            });

            cmds.send_event(PlayerLoggedInEvent::new(entity, profile.uuid, profile.username.clone()));

        }
    }
}
