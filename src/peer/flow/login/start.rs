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
use pipeworkmc_data::redacted::Redacted;
use pipeworkmc_packet::{
    c2s::{
        C2SPackets,
        login::{
            C2SLoginPackets,
            start::C2SLoginStartPacket
        }
    },
    s2c::login::encrypt_request::S2CLoginEncryptRequestPacket
};
use std::borrow::Cow;
use bevy_ecs::{
    message::MessageReader,
    query::With,
    system::{
        Query,
        Res
    }
};
use bevy_parmessagewriter::ParallelMessageWriter;
use openssl::rsa::Rsa;
use rand::RngCore;


/// Responds to login start requests.
pub(in crate::peer) fn begin_key_exchange(
    mut q_packet  : Query<(&mut PeerLoginFlow,), (With<Peer>,)>,
    mut mr_packet : MessageReader<PacketReceived>,
        mw_packet : ParallelMessageWriter<SendPacket>,
        r_options : Res<PeerOptions>
) {
    for m in mr_packet.read() {
        if let C2SPackets::Login(C2SLoginPackets::Start(
            C2SLoginStartPacket { username, uuid : _ }
        )) = &m.packet
            && let Ok((mut flow,)) = q_packet.get_mut(m.peer)
        {
            let PeerLoginFlow::Unstarted = &*flow else {
                mw_packet.write(SendPacket::new(m.peer).kick_login_failed("Login start invalid at this time"));
                continue;
            };

            // Create a new key pair which will be used to share a secret key.
            let     private_key    = Redacted::from(Rsa::generate(2048).unwrap());
            let     public_key_der = Redacted::from(unsafe { private_key.as_ref() }.public_key_to_der().unwrap());
            // Create a verify token which will be used to ensure that the secret key was exchanged properly.
            let mut verify_token   = [0u8; 4];
            rand::rng().fill_bytes(&mut verify_token);

            // Begin the key exchange process.
            mw_packet.write(SendPacket::new(m.peer).with_before_switch(S2CLoginEncryptRequestPacket {
                server_id       : r_options.server_id.clone(),
                public_key      : Redacted::from(Cow::Owned(unsafe { public_key_der.as_ref() }.clone())),
                verify_token,
                mojauth_enabled : {
                    #[cfg(feature = "mojauth")]
                    let me = r_options.mojauth_enabled;
                    #[cfg(not(feature = "mojauth"))]
                    let me = false;
                    me
                }
            }));

            *flow = PeerLoginFlow::KeyExchange {
                declared_username : username.clone(),
                private_key,
                #[cfg(feature = "mojauth")]
                public_key_der,
                verify_token
            };

        }
    }
}
