use super::PeerLoginFlow;
use crate::peer::{
    PeerAddress,
    PeerOptions,
    writer::PacketSender,
    event::{
        PacketReceived,
        SendPacket
    }
};
use crate::game::{
    login::{
        PlayerApproveLoginEvent,
        PlayerLoggedInEvent
    },
    character::{
        CharacterVisibility,
        player::{
            Dimension,
            IsHardcore,
            ViewDist,
            ReducedDebugInfo,
            NoRespawnScreen
        }
    }
};
use pipeworkmc_codec::meta::PacketState;
use pipeworkmc_data::{
    cat_variant::CatVariant,
    channel_data::ChannelData,
    character::CharacterId,
    chicken_variant::ChickenVariant,
    cow_variant::CowVariant,
    damage_type::DamageType,
    frog_variant::FrogVariant,
    game_mode::GameMode,
    known_pack::KnownPack,
    painting_variant::PaintingVariant,
    pig_variant::PigVariant,
    profile::AccountProfile,
    registry_entry::RegistryEntry,
    version::Version,
    wolf_variant::WolfVariant,
    wolf_sound_variant::WolfSoundVariant,
    worldgen::biome::WorldgenBiome
};
use pipeworkmc_packet::{
    c2s::{
        C2SPackets,
        login::{
            C2SLoginPackets,
            finish_acknowledged::C2SLoginFinishAcknowledgedPacket
        }
    },
    s2c::{
        login::finish::S2CLoginFinishPacket,
        config::{
            custom_payload::S2CConfigCustomPayloadPacket,
            known_packs::S2CConfigKnownPacksPacket,
            registry_data::S2CConfigRegistryDataPacket
        },
        play::{
            login::S2CPlayLoginPacket,
            game_event::S2CPlayGameEventPacket
        }
    }
};
use std::borrow::Cow;
use bevy_ecs::{
    event::{
        EventReader,
        EventWriter
    },
    query::{ Has, With },
    system::{
        Commands,
        Query,
        Res
    }
};


pub(in crate::peer) fn alert_approved_logins(
    mut q_peers   : Query<(&AccountProfile, &mut PeerLoginFlow,), (With<PeerAddress>,)>,
    mut er_login  : EventReader<PlayerApproveLoginEvent>,
    mut ew_packet : EventWriter<SendPacket>
) {
    for e in er_login.read() {
        if let Ok((profile, mut flow)) = q_peers.get_mut(e.entity()) {
            let PeerLoginFlow::Approval = &*flow else {
                continue;
            };

            ew_packet.write(SendPacket::new(e.entity())
                .with_before_switch(S2CLoginFinishPacket {
                    profile : AccountProfile::new(
                        profile.uuid,
                        profile.username.clone(),
                        None
                    )
                })
                .with_switch_state(PacketState::Config, true)
            );
            *flow = PeerLoginFlow::Acknowledge;
        }
    }
}


pub(in crate::peer) fn handle_login_acknowledge(
    mut cmds      : Commands,
    mut q_peers   : Query<(
        &mut PeerLoginFlow,
        &AccountProfile,
        &CharacterId,
        &Dimension,
        Has<IsHardcore>,
        &ViewDist,
        Has<ReducedDebugInfo>,
        Has<NoRespawnScreen>,
        &GameMode,
        &mut CharacterVisibility
    ), (With<PeerAddress>,)>,
    mut er_packet : EventReader<PacketReceived>,
    mut ew_packet : EventWriter<SendPacket>,
    mut ew_login  : EventWriter<PlayerLoggedInEvent>,
        r_options : Res<PeerOptions>
) {
    for e in er_packet.read() {
        if let C2SPackets::Login(C2SLoginPackets::FinishAcknowledged(
            C2SLoginFinishAcknowledgedPacket { }
        )) = e.packet() {
            if let Ok((
                mut flow,
                profile,
                chid,
                dimension,
                is_hardcore,
                view_dist,
                reduced_debug_info,
                no_respawn_screen,
                game_mode,
                mut vis,
            )) = q_peers.get_mut(e.entity()) {
                let PeerLoginFlow::Acknowledge = &*flow else {
                    ew_packet.write(SendPacket::new(e.entity()).kick_login_failed("Login acknowledgement invalid at this time"));
                    continue;
                };

                *flow = PeerLoginFlow::Done;
                cmds.entity(e.entity()).remove::<PeerLoginFlow>();

                ew_packet.write(SendPacket::new(e.entity()).with_before_switch(
                    S2CConfigCustomPayloadPacket { data : ChannelData::Brand {
                        brand : Cow::Borrowed(&r_options.server_brand)
                    } }
                ));

                ew_packet.write(SendPacket::new(e.entity()).with_before_switch(
                    S2CConfigKnownPacksPacket { known_packs : Cow::Borrowed(&[ KnownPack {
                        namespace : Cow::Borrowed("minecraft"),
                        id        : Cow::Borrowed("core"),
                        version   : Cow::Borrowed(Version::CURRENT.latest_name())
                    } ]) }
                ));

                // TODO: Make these customisable.
                ew_packet.write(SendPacket::new(e.entity()).with_before_switch(
                    S2CConfigRegistryDataPacket::from(CatVariant::VANILLA_REGISTRY_ENTRIES))
                );
                ew_packet.write(SendPacket::new(e.entity()).with_before_switch(
                    S2CConfigRegistryDataPacket::from(ChickenVariant::VANILLA_REGISTRY_ENTRIES))
                );
                ew_packet.write(SendPacket::new(e.entity()).with_before_switch(
                    S2CConfigRegistryDataPacket::from(CowVariant::VANILLA_REGISTRY_ENTRIES))
                );
                ew_packet.write(SendPacket::new(e.entity()).with_before_switch(
                    S2CConfigRegistryDataPacket::from(DamageType::VANILLA_REGISTRY_ENTRIES))
                );
                ew_packet.write(SendPacket::new(e.entity()).with_before_switch(
                    S2CConfigRegistryDataPacket::from(FrogVariant::VANILLA_REGISTRY_ENTRIES))
                );
                ew_packet.write(SendPacket::new(e.entity()).with_before_switch(
                    S2CConfigRegistryDataPacket::from(PaintingVariant::VANILLA_REGISTRY_ENTRIES))
                );
                ew_packet.write(SendPacket::new(e.entity()).with_before_switch(
                    S2CConfigRegistryDataPacket::from(PigVariant::VANILLA_REGISTRY_ENTRIES))
                );
                ew_packet.write(SendPacket::new(e.entity()).with_before_switch(
                    S2CConfigRegistryDataPacket::from(WolfVariant::VANILLA_REGISTRY_ENTRIES))
                );
                ew_packet.write(SendPacket::new(e.entity()).with_before_switch(
                    S2CConfigRegistryDataPacket::from(WolfSoundVariant::VANILLA_REGISTRY_ENTRIES))
                );
                ew_packet.write(SendPacket::new(e.entity()).with_before_switch(
                    S2CConfigRegistryDataPacket::from(WorldgenBiome::VANILLA_REGISTRY_ENTRIES))
                );

                ew_packet.write(SendPacket::new(e.entity()).with_before_switch(
                    S2CConfigRegistryDataPacket::from(&[
                        RegistryEntry { id : dimension.id.clone(), data : &dimension.dim_type }
                    ])
                ));

                ew_packet.write(SendPacket::new(e.entity()).with(
                    S2CPlayLoginPacket {
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
                    }
                ));
                vis.show_to(e.entity()); // Make sure the player can see themself.
                ew_packet.write(SendPacket::new(e.entity()).with_before_switch(
                    S2CPlayGameEventPacket::WaitForChunks
                ));

                ew_login.write(PlayerLoggedInEvent::new(
                    e.entity(), profile.uuid, profile.username.clone()
                ));

            }
        }
    }
}
