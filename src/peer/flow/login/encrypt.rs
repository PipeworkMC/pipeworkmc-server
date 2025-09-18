use super::{
    PeerLoginFlow,
    mojauth::build_mojauth_uri
};
use crate::peer::{
    PeerAddress,
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
    event::{
        EventReader,
        EventWriter
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


const OFFLINE_NAMESPACE : Uuid = Uuid::from_bytes([b'P', b'i', b'p', b'e', b'w', b'o', b'r', b'k', b'_', b'O', b'f', b'f', b'l', b'i', b'n', b'e']);


pub(in crate::peer) fn finish_key_exchange_and_check_mojauth(
    mut cmds      : Commands,
    mut q_peers   : Query<(&mut PeerStreamReader, &mut PeerStreamWriter, &mut PeerLoginFlow,), (With<PeerAddress>,)>,
    mut er_packet : EventReader<PacketReceived>,
    mut ew_packet : EventWriter<SendPacket>,
    mut ew_login  : EventWriter<PlayerRequestLoginEvent>,
        r_options : Res<PeerOptions>,
        r_chid    : Res<NextCharacterId>
) {
    for e in er_packet.read() {
        if let C2SPackets::Login(C2SLoginPackets::EncryptResponse(
            C2SLoginEncryptResponsePacket { encrypted_secret_key, encrypted_vtoken }
        )) = e.packet()
            && let Ok((mut reader, mut writer, mut flow,)) = q_peers.get_mut(e.entity())
        {
            let PeerLoginFlow::KeyExchange { declared_username, private_key, public_key_der, verify_token } = &*flow else {
                ew_packet.write(SendPacket::new(e.entity()).kick_login_failed("Key exchange invalid at this time"));
                continue;
            };

            // Check verify token.
            let mut decrypted_vtoken = [0u8; 256];
            let Ok(vtoken_size) = unsafe { private_key.as_ref() }.private_decrypt(encrypted_vtoken, &mut decrypted_vtoken, Padding::PKCS1) else {
                ew_packet.write(SendPacket::new(e.entity()).kick_login_failed("Public key exchange failed"));
                continue;
            };
            if (*verify_token != decrypted_vtoken[0..vtoken_size]) {
                ew_packet.write(SendPacket::new(e.entity()).kick_login_failed("Public key exchange verification failed"));
                continue;
            }

            // Decrypt secret key.
            let mut decrypted_secret_key = Redacted::from([0u8; 256]);
            let Ok(secret_key_size) = unsafe { private_key.as_ref() }.private_decrypt(unsafe { encrypted_secret_key.as_ref() }, unsafe { decrypted_secret_key.as_mut() }, Padding::PKCS1) else {
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

            // Begin mojauth if enabled.
            if (r_options.mojauth_enabled) {
                let (url_buf, url_len,) = build_mojauth_uri(
                    &r_options.server_id,
                    &decrypted_secret_key,
                    &public_key_der,
                    &declared_username
                );
                *flow = PeerLoginFlow::Mojauth { task : IoTaskPool::get().spawn(async move {
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
                ew_login.write(PlayerRequestLoginEvent::new(
                    e.entity(), profile.uuid, profile.username.clone()
                ));
                cmds.entity(e.entity()).insert((
                    profile,
                    r_chid.next(),
                    PlayerBundle::default()
                ));
                *flow = PeerLoginFlow::Approval;
            }

        }
    }
}
