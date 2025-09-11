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


const MOJAUTH_URL_PREFIX   : &str = "https://sessionserver.mojang.com/session/minecraft/hasJoined?username=";
const MOJAUTH_URL_SERVERID : &str = "&serverId=";

const OFFLINE_NAMESPACE : Uuid = Uuid::from_bytes([b'P', b'i', b'p', b'e', b'w', b'o', b'r', b'k', b'_', b'O', b'f', b'f', b'l', b'i', b'n', b'e']);


#[derive(Component, Default, Debug)]
pub(in crate::conn) struct ConnPeerLoginFlow {
    declared_username : Option<BoundedString<16>>,
    exchanging_key    : Option<ExchangingKey>,
    mojauth_task      : Option<Task<surf::Result<AccountProfile>>>,
    approved          : bool
}

#[derive(Debug)]
struct ExchangingKey {
    rsa     : Redacted<Rsa<Private>>,
    pkeyder : Redacted<Vec<u8>>,
    vtoken  : [u8; 4]
}


pub(in crate::conn) fn handle_login_flow(
    mut cmds      : Commands,
    mut q_peers   : Query<(
        Entity,
        &mut ConnPeerReader,
        &mut ConnPeerWriter,
        &mut ConnPeerSender,
        &mut ConnPeerLoginFlow,
    )>,
    mut er_login  : EventReader<IncomingLoginPacketEvent>,
        r_options : Res<ConnOptions>,
    mut r_chid    : ResMut<NextCharacterId>
) {
    for event in er_login.read() {
        if let Ok((
            entity,
            mut reader,
            mut writer,
            mut sender,
            mut login_flow,
        )) = q_peers.get_mut(event.peer()) {
            if (sender.is_disconnecting()) { continue; }
            match (event.packet()) {


                C2SLoginPackets::Start(C2SLoginStartPacket { username, uuid : _ }) => {
                    if (login_flow.declared_username.is_some()) {
                        sender.kick_login_failed("Client-side profile already declared");
                        continue;
                    }
                    login_flow.declared_username = Some(username.clone());

                    let     rsa     = Redacted::from(Rsa::generate(2048).unwrap());
                    let     pkeyder = Redacted::from(unsafe { rsa.as_ref() }.public_key_to_der().unwrap());
                    let mut vtoken  = [0u8; 4];
                    rand::rng().fill_bytes(&mut vtoken);
                    sender.send(S2CLoginEncryptRequestPacket {
                        server_id       : r_options.server_id.clone(),
                        public_key      : Redacted::from(Cow::Owned(unsafe { pkeyder.as_ref() }.clone())),
                        verify_token    : vtoken,
                        mojauth_enabled : r_options.mojauth_enabled
                    });

                    login_flow.exchanging_key = Some(ExchangingKey { rsa, pkeyder, vtoken });
                },


                C2SLoginPackets::EncryptResponse(C2SLoginEncryptResponsePacket { encrypted_secret_key, encrypted_vtoken }) => {
                    let Some(ExchangingKey { rsa, pkeyder, vtoken }) = login_flow.exchanging_key.take() else {
                        sender.kick_login_failed("Invalid public key exchange");
                        continue;
                    };
                    let Some(declared_username) = &login_flow.declared_username
                        else { unsafe { unreachable_unchecked() } };

                    // Check verify token.
                    let mut decrypted_vtoken = [0u8; 256];
                    let Ok(vtoken_size) = unsafe { rsa.as_ref() }.private_decrypt(encrypted_vtoken, &mut decrypted_vtoken, Padding::PKCS1) else {
                        sender.kick_login_failed("Public key exchange failed");
                        continue;
                    };
                    if (vtoken != decrypted_vtoken[0..vtoken_size]) {
                        sender.kick_login_failed("Public key exchange verification failed");
                        continue;
                    }

                    // Decrypt secret key.
                    let mut decrypted_secret_key = Redacted::from([0u8; 256]);
                    let Ok(secret_key_size) = unsafe { rsa.as_ref() }.private_decrypt(unsafe { encrypted_secret_key.as_ref() }, unsafe { decrypted_secret_key.as_mut() }, Padding::PKCS1) else {
                        sender.kick_login_failed("Secret key exchange failed");
                        continue;
                    };
                    let decrypted_secret_key = Redacted::from(&unsafe { decrypted_secret_key.as_ref() }[0..secret_key_size]);

                    // Enable encryption.
                    let cipher = Cipher::aes_128_cfb8();

                    let Ok(decrypter) = Crypter::new(cipher, CrypterMode::Decrypt, unsafe { decrypted_secret_key.as_ref() }, Some(unsafe { decrypted_secret_key.as_ref() }) ) else {
                        sender.kick_login_failed("Invalid secret key received");
                        continue;
                    };
                    reader.decrypter = Some(Redacted::from(decrypter));

                    let Ok(encrypter) = Crypter::new(cipher, CrypterMode::Encrypt, unsafe { decrypted_secret_key.as_ref() }, Some(unsafe { decrypted_secret_key.as_ref() }) ) else {
                        sender.kick_login_failed("Invalid secret key received");
                        continue;
                    };
                    writer.encrypter = Some(Redacted::from(encrypter));

                    if (r_options.mojauth_enabled) { // Fetch account information.

                        let (url_buf, url_len,) = build_mojauth_uri(
                            &r_options.server_id,
                            &decrypted_secret_key,
                            &pkeyder,
                            declared_username
                        );
                        login_flow.mojauth_task = Some(IoTaskPool::get().spawn(async move {
                            let url = unsafe { str::from_utf8_unchecked(url_buf.get_unchecked(0..url_len)) };
                            match (surf::get(url).send().await) {
                                Ok(mut response) => response.body_json::<AccountProfile>().await,
                                Err(err)         => Err(err)
                            }
                        }));

                    } else {
                        let profile = AccountProfile {
                            uuid     : Uuid::new_v3(&OFFLINE_NAMESPACE, declared_username.as_bytes()),
                            username : declared_username.clone(),
                            skin     : None
                        };

                        cmds.send_event(PlayerRequestLoginEvent::new(entity, profile.uuid, profile.username.clone()));
                        let mut ecmds = cmds.entity(entity);
                        ecmds.insert((
                            r_chid.next(),
                            profile,
                            Dimension::default(),
                            ViewDistance::default(),
                            GameMode::Adventure
                        ));
                    }

                },


                C2SLoginPackets::FinishAcknowledged(C2SLoginFinishAcknowledgedPacket {}) => { }


            }
        }
    }
}


pub(in crate::conn) fn poll_mojauths_tasks(
    mut cmds    : Commands,
    mut q_peers : Query<(Entity, &mut ConnPeerLoginFlow,)>,
) {
    for (entity, mut login_flow,) in &mut q_peers {
        if let Some(mojauth_task) = &mut login_flow.mojauth_task
            && let Some(response) = futures::check_ready(mojauth_task)
        {
            login_flow.mojauth_task = None;
            match (response) {
                Ok(profile) => {
                    cmds.send_event(PlayerRequestLoginEvent::new(entity, profile.uuid, profile.username.clone()));
                    let mut ecmds = cmds.entity(entity);
                    ecmds.insert(profile);
                },
                Err(err) => panic!("{err:?}") // TODO: Error handler.
            }
        }
    }
}
pub(in crate::conn) fn is_mojauth_enabled(
    r_options : Res<ConnOptions>
) -> bool { r_options.mojauth_enabled }


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


fn build_mojauth_uri(
    server_id            : &str,
    decrypted_secret_key : &Redacted<&[u8]>,
    pkeyder              : &Redacted<Vec<u8>>,
    declared_username    : &str
) -> ([u8; MOJAUTH_URL_PREFIX.len() + 16 + MOJAUTH_URL_SERVERID.len() + 41], usize,) {
    // Build server ID.
    let mut sha = Sha1::new();
    sha.update(server_id.as_bytes());
    sha.update(unsafe { decrypted_secret_key.as_ref() });
    sha.update(unsafe { pkeyder.as_ref() });
    let     sha_in_20 = sha.finish();

    let mut sha_in_32   = [0u8; 32];
    unsafe { ptr::copy_nonoverlapping(sha_in_20.as_ptr(), sha_in_32.as_mut_ptr(), 20); }
    let     sha_in_i256 = i256::from_be_bytes(sha_in_32);
    let mut sha_buf     = [0u8; 40];
    if (sha_in_i256 >= 0) {
        _ = hex::encode_to_slice(sha_in_20, &mut sha_buf);
    } else {
        let neg_sha_in_32 = (-sha_in_i256).to_be_bytes();
        // SAFETY: sha_in_32 bytes has 32 items.
        _ = hex::encode_to_slice(unsafe { neg_sha_in_32.get_unchecked(0..20) }, &mut sha_buf);
    }
    // SAFETY: sha_buf has 40 items.
    let sha_buf = unsafe { sha_buf.get_unchecked((sha_buf.iter().position(|&x| x != b'0').unwrap_or(39))..40) };

    // Build mojauth URI.
    let mut url_buf = [0u8; MOJAUTH_URL_PREFIX.len() + 16 + MOJAUTH_URL_SERVERID.len() + 41];
    let mut url_ptr = 0;
    // SAFETY: url_buf has enough space for `MOJAUTH_URL_PREFIX`, `declared_username`, `MOJAUTH_URL_SERVERID`, and `sha_buf`.
    //         None are written to overlap each other.
    //         declared_username can not be longer than 16 bytes (checked above).
    {
        unsafe { ptr::copy_nonoverlapping(MOJAUTH_URL_PREFIX.as_ptr(), url_buf.as_mut_ptr().byte_add(url_ptr), MOJAUTH_URL_PREFIX.len()); }
        url_ptr += MOJAUTH_URL_PREFIX.len();
        unsafe { ptr::copy_nonoverlapping(declared_username.as_ptr(), url_buf.as_mut_ptr().byte_add(url_ptr), declared_username.len()); }
        url_ptr += declared_username.len();
        unsafe { ptr::copy_nonoverlapping(MOJAUTH_URL_SERVERID.as_ptr(), url_buf.as_mut_ptr().byte_add(url_ptr), MOJAUTH_URL_SERVERID.len()); }
        url_ptr += MOJAUTH_URL_SERVERID.len();
        if (sha_in_i256 < 0) {
            unsafe { url_buf.as_mut_ptr().byte_add(url_ptr).write(b'-'); }
            url_ptr += 1;
        }
        unsafe { ptr::copy_nonoverlapping(sha_buf.as_ptr(), url_buf.as_mut_ptr().byte_add(url_ptr), sha_buf.len()); }
        url_ptr += sha_buf.len();
    }

    (url_buf, url_ptr,)
}
