use super::{
    PeerLoginFlow,
    mojauth::build_mojauth_uri
};
use crate::peer::{
    Peer,
    PeerOptions,
    reader::PeerStreamReader,
    writer::{
        PeerStreamWriter,
        PacketSender
    },
    message::{
        PacketReceived,
        SendPacket
    }
};
use crate::game::{
    login::PlayerLoginRequest,
    character::{
        player::PlayerCharacterBundle,
        vis::VisibleCharacters
    }
};
use pipeworkmc_codec::meta::PacketState;
use pipeworkmc_data::{
    profile::AccountProfile,
    redacted::Redacted,
    uuid::Uuid
};
use pipeworkmc_packet::{
    s2c::login::finish::S2CLoginFinishPacket,
    c2s::{
        C2SPackets,
        login::{
            C2SLoginPackets,
            encrypt_response::C2SLoginEncryptResponsePacket
        }
    }
};
use bevy_callback::Callback;
use bevy_ecs::{
    message::{
        MessageWriter,
        MessageReader
    },
    query::With,
    system::{
        Commands,
        Query,
        Res
    }
};
use bevy_tasks::IoTaskPool;
use openssl::{
    rsa::Padding,
    symm::{
        Cipher,
        Crypter,
        Mode as CrypterMode
    }
};


const OFFLINE_NAMESPACE : Uuid = Uuid::from_bytes(*b"Pipework_Offline");


/// Finish the key exchange process.
pub(in crate::peer) fn finish_key_exchange_and_check_mojauth(
    mut cmds      : Commands,
    mut q_peers   : Query<(&mut PeerStreamReader, &mut PeerStreamWriter, &mut PeerLoginFlow,), (With<Peer>,)>,
    mut mr_packet : MessageReader<PacketReceived>,
    mut mw_packet : MessageWriter<SendPacket>,
        r_options : Res<PeerOptions>,
    mut c_login   : Callback<PlayerLoginRequest>
) {
    for m in mr_packet.read() {
        if let C2SPackets::Login(C2SLoginPackets::EncryptResponse(
            C2SLoginEncryptResponsePacket { encrypted_secret_key, encrypted_vtoken }
        )) = &m.packet
            && let Ok((mut reader, mut writer, mut flow,)) = q_peers.get_mut(m.peer)
        {
            let PeerLoginFlow::KeyExchange { declared_username, private_key, public_key_der, verify_token } = &*flow else {
                mw_packet.write(SendPacket::new(m.peer).kick_login_failed("Key exchange invalid at this time"));
                continue;
            };

            // Check verify token.
            let mut decrypted_vtoken = [0u8; 256];
            let Ok(vtoken_size) = unsafe { private_key.as_ref() }.private_decrypt(encrypted_vtoken, &mut decrypted_vtoken, Padding::PKCS1) else {
                mw_packet.write(SendPacket::new(m.peer).kick_login_failed("Public key exchange failed"));
                continue;
            };
            if (*verify_token != decrypted_vtoken[0..vtoken_size]) {
                mw_packet.write(SendPacket::new(m.peer).kick_login_failed("Public key exchange verification failed"));
                continue;
            }

            // Decrypt secret key.
            let mut decrypted_secret_key = Redacted::from([0u8; 256]);
            let Ok(secret_key_size) = unsafe { private_key.as_ref() }.private_decrypt(unsafe { encrypted_secret_key.as_ref() }, unsafe { decrypted_secret_key.as_mut() }, Padding::PKCS1) else {
                mw_packet.write(SendPacket::new(m.peer).kick_login_failed("Secret key exchange failed"));
                continue;
            };
            let decrypted_secret_key = Redacted::from(&unsafe { decrypted_secret_key.as_ref() }[0..secret_key_size]);

            // Enable encryption.
            let cipher = Cipher::aes_128_cfb8();

            let Ok(decrypter) = Crypter::new(cipher, CrypterMode::Decrypt, unsafe { decrypted_secret_key.as_ref() }, Some(unsafe { decrypted_secret_key.as_ref() }) ) else {
                mw_packet.write(SendPacket::new(m.peer).kick_login_failed("Invalid secret key received"));
                continue;
            };
            reader.set_decrypter(Redacted::from(decrypter));

            let Ok(encrypter) = Crypter::new(cipher, CrypterMode::Encrypt, unsafe { decrypted_secret_key.as_ref() }, Some(unsafe { decrypted_secret_key.as_ref() }) ) else {
                mw_packet.write(SendPacket::new(m.peer).kick_login_failed("Invalid secret key received"));
                continue;
            };
            writer.set_encrypter(Redacted::from(encrypter));

            // Begin mojauth if enabled.
            if (r_options.mojauth_enabled) {
                let (url_buf, url_len,) = build_mojauth_uri(
                    &r_options.server_id,
                    &decrypted_secret_key,
                    public_key_der,
                    declared_username
                );
                *flow = PeerLoginFlow::Mojauth { task : IoTaskPool::get().spawn(async move {
                    // SAFETY: `build_mojauth_uri` only returns valid UTF8.
                    //         `url_len` is always less than `url_buf.len()`.
                    let url = unsafe { str::from_utf8_unchecked(url_buf.get_unchecked(0..url_len)) };
                    match (surf::get(url).send().await) {
                        Ok(mut response) => response.body_json::<AccountProfile>().await,
                        Err(err)         => Err(err)
                    }
                }) };
            }
            // If mojauth disabled, skip to requesting approval.
            else {
                let profile = AccountProfile {
                    uuid     : Uuid::new_v3(&OFFLINE_NAMESPACE, declared_username.as_bytes()),
                    username : declared_username.clone(),
                    skin     : None
                };
                let approval = c_login.request(PlayerLoginRequest {
                    peer     : m.peer,
                    uuid     : profile.uuid,
                    username : profile.username.clone()
                });
                match (approval) {
                    Ok(()) => {
                        mw_packet.write(SendPacket::new(m.peer)
                            .with_before_switch(S2CLoginFinishPacket {
                                profile : profile.clone()
                            })
                            .with_switch_state(PacketState::Config, true)
                        );
                        cmds.entity(m.peer).insert((
                            profile,
                            PlayerCharacterBundle::default(),
                            VisibleCharacters::new(m.peer)
                        ));
                        *flow = PeerLoginFlow::Acknowledge;
                    },
                    Err(reason) => {
                        mw_packet.write(SendPacket::new(m.peer).kick(&reason));
                        *flow = PeerLoginFlow::Done;
                    }
                }
            }

        }
    }
}
