use super::PeerLoginFlow;
use crate::peer::{
    Peer,
    PeerOptions,
    writer::PacketSender,
    message::{
        PacketReceived,
        SendPacket
    }
};
use crate::game::{
    login::PlayerLoggedInMessage,
    character::{
        CharacterVisibility,
        player::{
            ReadyPlayerCharacter,
            Dimension,
            IsHardcore,
            ViewDist,
            ReducedDebugInfo,
            NoRespawnScreen
        }
    }
};
use pipeworkmc_data::{
    cat_variant::CatVariant,
    channel_data::ChannelData,
    chicken_variant::ChickenVariant,
    character::CharacterId,
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
    message::{
        MessageReader,
        MessageWriter
    },
    query::{ Has, With },
    system::{
        Commands,
        Query,
        Res
    }
};


/// Set up registries and other global synchronised data after a peer has logged in.
pub(in crate::peer) fn handle_login_acknowledge(
    mut cmds      : Commands,
    mut q_peers   : Query<(
        &mut PeerLoginFlow,
        &AccountProfile,
        &Dimension,
        Has<IsHardcore>,
        &ViewDist,
        Has<ReducedDebugInfo>,
        Has<NoRespawnScreen>,
        &GameMode,
        &mut CharacterVisibility
    ), (With<Peer>,)>,
    mut mr_packet : MessageReader<PacketReceived>,
    mut mw_packet : MessageWriter<SendPacket>,
    mut mw_login  : MessageWriter<PlayerLoggedInMessage>,
        r_options : Res<PeerOptions>
) {
    for m in mr_packet.read() {
        if let C2SPackets::Login(C2SLoginPackets::FinishAcknowledged(C2SLoginFinishAcknowledgedPacket { })) = m.packet // TODO: Hold this until at least 2 cycles after the login request was called.
            && let Ok((
                mut flow,
                profile,
                dimension,
                is_hardcore,
                view_dist,
                reduced_debug_info,
                no_respawn_screen,
                game_mode,
                mut vis,
            )) = q_peers.get_mut(m.peer)
        {
            let PeerLoginFlow::Acknowledge = &*flow else {
                mw_packet.write(SendPacket::new(m.peer).kick_login_failed("Login acknowledgement invalid at this time"));
                continue;
            };

            *flow = PeerLoginFlow::Done;
            cmds.entity(m.peer).remove::<PeerLoginFlow>();

            // Send server brand.
            mw_packet.write(SendPacket::new(m.peer).with_before_switch(
                S2CConfigCustomPayloadPacket { data : ChannelData::Brand {
                    brand : Cow::Borrowed(&r_options.server_brand)
                } }
            ));

            // Send server known packs.
            mw_packet.write(SendPacket::new(m.peer).with_before_switch(
                S2CConfigKnownPacksPacket { known_packs : Cow::Borrowed(&[ KnownPack {
                    namespace : Cow::Borrowed("minecraft"),
                    id        : Cow::Borrowed("core"),
                    version   : Cow::Borrowed(Version::CURRENT.latest_name())
                } ]) }
            ));

            // Send registries.
            // TODO: Make these customisable.
            mw_packet.write(SendPacket::new(m.peer).with_before_switch(
                S2CConfigRegistryDataPacket::from(CatVariant::VANILLA_REGISTRY_ENTRIES))
            );
            mw_packet.write(SendPacket::new(m.peer).with_before_switch(
                S2CConfigRegistryDataPacket::from(ChickenVariant::VANILLA_REGISTRY_ENTRIES))
            );
            mw_packet.write(SendPacket::new(m.peer).with_before_switch(
                S2CConfigRegistryDataPacket::from(CowVariant::VANILLA_REGISTRY_ENTRIES))
            );
            mw_packet.write(SendPacket::new(m.peer).with_before_switch(
                S2CConfigRegistryDataPacket::from(DamageType::VANILLA_REGISTRY_ENTRIES))
            );
            mw_packet.write(SendPacket::new(m.peer).with_before_switch(
                S2CConfigRegistryDataPacket::from(FrogVariant::VANILLA_REGISTRY_ENTRIES))
            );
            mw_packet.write(SendPacket::new(m.peer).with_before_switch(
                S2CConfigRegistryDataPacket::from(PaintingVariant::VANILLA_REGISTRY_ENTRIES))
            );
            mw_packet.write(SendPacket::new(m.peer).with_before_switch(
                S2CConfigRegistryDataPacket::from(PigVariant::VANILLA_REGISTRY_ENTRIES))
            );
            mw_packet.write(SendPacket::new(m.peer).with_before_switch(
                S2CConfigRegistryDataPacket::from(WolfVariant::VANILLA_REGISTRY_ENTRIES))
            );
            mw_packet.write(SendPacket::new(m.peer).with_before_switch(
                S2CConfigRegistryDataPacket::from(WolfSoundVariant::VANILLA_REGISTRY_ENTRIES))
            );
            mw_packet.write(SendPacket::new(m.peer).with_before_switch(
                S2CConfigRegistryDataPacket::from(WorldgenBiome::VANILLA_REGISTRY_ENTRIES))
            );

            mw_packet.write(SendPacket::new(m.peer).with_before_switch(
                S2CConfigRegistryDataPacket::from(&[
                    RegistryEntry { id : dimension.id.clone(), data : &dimension.dim_type }
                ])
            ));

            // Finalise login.
            mw_packet.write(SendPacket::new(m.peer).with(
                S2CPlayLoginPacket {
                    eid                   : CharacterId(0),
                    hardcore              : is_hardcore,
                    all_dim_ids           : Cow::Owned(vec![dimension.id.clone()]),
                    max_players           : 0,
                    view_dist             : view_dist.as_u8() as u32,
                    sim_dist              : 32,
                    reduced_debug_info,
                    respawn_screen        : ! no_respawn_screen,
                    limited_crafting      : true,
                    dim_type              : 0,
                    dim_id                : dimension.id.clone(),
                    hashed_seed           : dimension.hashed_seed,
                    game_mode             : *game_mode,
                    prev_game_mode        : None,
                    is_debug_world        : dimension.is_debug,
                    is_flat_world         : dimension.is_flat,
                    death_location        : None,
                    portal_cooldown       : 0,
                    sea_level             : dimension.sea_level,
                    requires_chat_signing : false
                }
            ));
            vis.show_to(m.peer); // Make sure the player can see themself.
            mw_packet.write(SendPacket::new(m.peer).with_before_switch(
                S2CPlayGameEventPacket::WaitForChunks
            ));

            mw_login.write(PlayerLoggedInMessage {
                peer     : m.peer,
                uuid     : profile.uuid,
                username : profile.username.clone()
            });
            cmds.entity(m.peer).insert(ReadyPlayerCharacter);

        }
    }
}
