use super::mojauth::{
    MojauthTask,
    build_mojauth_uri
};
use crate::peer::{
    PeerOptions,
    reader::PeerStreamReader,
    writer::{
        PeerStreamWriter,
        PacketSender
    },
    event::{
        PacketReceived,
        SendPacket
    }
};
use crate::game::player::{
    login::PlayerRequestLoginEvent,
    data::PlayerBundle
};
use pipeworkmc_data::{
    bounded_string::BoundedString,
    character::NextCharacterId,
    profile::AccountProfile,
    redacted::Redacted,
    uuid::Uuid
};
use pipeworkmc_packet::c2s::{
    C2SPackets,
    login::{
        C2SLoginPackets,
        encrypt_response::C2SLoginEncryptResponsePacket
    }
};
use bevy_ecs::{
    component::Component,
    event::{
        EventReader,
        EventWriter
    },
    system::{
        Commands,
        Query,
        Res
    }
};
use bevy_tasks::IoTaskPool;
use openssl::{
    pkey::Private,
    rsa::{ Padding, Rsa },
    symm::{
        Cipher,
        Crypter,
        Mode as CrypterMode
    }
};


const OFFLINE_NAMESPACE : Uuid = Uuid::from_bytes([b'P', b'i', b'p', b'e', b'w', b'o', b'r', b'k', b'_', b'O', b'f', b'f', b'l', b'i', b'n', b'e']);


#[derive(Component)]
#[component(storage = "SparseSet")]
pub(in crate::peer) struct KeyExchange {
    pub(super) declared_username : BoundedString<16>,
    pub(super) private_key       : Redacted<Rsa<Private>>,
    pub(super) public_key_der    : Redacted<Vec<u8>>,
    pub(super) verify_token      : [u8; 4],
    pub(super) invalidated       : bool
}


pub(in crate::peer) fn finish_key_exchange_and_check_mojauth(
    mut cmds      : Commands,
    mut q_peers   : Query<(&mut PeerStreamReader, &mut PeerStreamWriter, &mut KeyExchange,)>,
    mut er_packet : EventReader<PacketReceived>,
    mut ew_packet : EventWriter<SendPacket>,
    mut ew_login  : EventWriter<PlayerRequestLoginEvent>,
        r_options : Res<PeerOptions>,
        r_chid    : Res<NextCharacterId>
) {
    for e in er_packet.read() {
        if let C2SPackets::Login(C2SLoginPackets::EncryptResponse(C2SLoginEncryptResponsePacket { encrypted_secret_key, encrypted_vtoken })) = e.packet()
            && let Ok((mut reader, mut writer, mut keyex,)) = q_peers.get_mut(e.entity())
        {
            if (keyex.invalidated) {
                ew_packet.write(SendPacket::new(e.entity()).kick_login_failed("Key exchange already done"));
                continue;
            }
            keyex.invalidated = true;

            // Check verify token.
            let mut decrypted_vtoken = [0u8; 256];
            let Ok(vtoken_size) = unsafe { keyex.private_key.as_ref() }.private_decrypt(encrypted_vtoken, &mut decrypted_vtoken, Padding::PKCS1) else {
                ew_packet.write(SendPacket::new(e.entity()).kick_login_failed("Public key exchange failed"));
                continue;
            };
            if (keyex.verify_token != decrypted_vtoken[0..vtoken_size]) {
                ew_packet.write(SendPacket::new(e.entity()).kick_login_failed("Public key exchange verification failed"));
                continue;
            }

            // Decrypt secret key.
            let mut decrypted_secret_key = Redacted::from([0u8; 256]);
            let Ok(secret_key_size) = unsafe { keyex.private_key.as_ref() }.private_decrypt(unsafe { encrypted_secret_key.as_ref() }, unsafe { decrypted_secret_key.as_mut() }, Padding::PKCS1) else {
                ew_packet.write(SendPacket::new(e.entity()).kick_login_failed("Secret key exchange failed"));
                continue;
            };
            let decrypted_secret_key = Redacted::from(&unsafe { decrypted_secret_key.as_ref() }[0..secret_key_size]);

            // Enable encryption.
            let cipher = Cipher::aes_128_cfb8();

            let Ok(decrypter) = Crypter::new(cipher, CrypterMode::Decrypt, unsafe { decrypted_secret_key.as_ref() }, Some(unsafe { decrypted_secret_key.as_ref() }) ) else {
                ew_packet.write(SendPacket::new(e.entity()).kick_login_failed("Invalid secret key received"));
                continue;
            };
            reader.set_decrypter(Redacted::from(decrypter));

            let Ok(encrypter) = Crypter::new(cipher, CrypterMode::Encrypt, unsafe { decrypted_secret_key.as_ref() }, Some(unsafe { decrypted_secret_key.as_ref() }) ) else {
                ew_packet.write(SendPacket::new(e.entity()).kick_login_failed("Invalid secret key received"));
                continue;
            };
            writer.set_encrypter(Redacted::from(encrypter));

            let mut ecmds = cmds.entity(e.entity());
            ecmds.remove::<KeyExchange>();

            // Begin mojauth if enabled.
            if (r_options.mojauth_enabled) {
                let (url_buf, url_len,) = build_mojauth_uri(
                    &r_options.server_id,
                    &decrypted_secret_key,
                    &keyex.public_key_der,
                    &keyex.declared_username
                );
                ecmds.insert(MojauthTask {
                    task        : IoTaskPool::get().spawn(async move {
                        let url = unsafe { str::from_utf8_unchecked(url_buf.get_unchecked(0..url_len)) };
                        match (surf::get(url).send().await) {
                            Ok(mut response) => response.body_json::<AccountProfile>().await,
                            Err(err)         => Err(err)
                        }
                    }),
                    invalidated : false
                });
            }
            // If mojauth disabled, skip to requesting approval.
            else {
                let profile = AccountProfile {
                    uuid     : Uuid::new_v3(&OFFLINE_NAMESPACE, keyex.declared_username.as_bytes()),
                    username : keyex.declared_username.clone(),
                    skin     : None
                };
                ew_login.write(PlayerRequestLoginEvent::new(
                    e.entity(), profile.uuid, profile.username.clone()
                ));
                ecmds.insert((
                    profile,
                    r_chid.next(),
                    PlayerBundle::default()
                ));
            }

        }
    }
}
