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
use crate::ecs::ParallelEventWriter;
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
    event::EventReader,
    query::With,
    system::{
        Query,
        Res
    }
};
use openssl::rsa::Rsa;
use rand::RngCore;


pub(in crate::peer) fn begin_key_exchange(
    mut q_packet  : Query<(&mut PeerLoginFlow,), (With<PeerAddress>,)>,
    mut er_packet : EventReader<PacketReceived>,
        ew_packet : ParallelEventWriter<SendPacket>,
        r_options : Res<PeerOptions>
) {
    for e in er_packet.read() {
        if let C2SPackets::Login(C2SLoginPackets::Start(
            C2SLoginStartPacket { username, uuid : _ }
        )) = e.packet()
            && let Ok((mut flow,)) = q_packet.get_mut(e.entity())
        {
            let PeerLoginFlow::Unstarted = &*flow else {
                ew_packet.write(SendPacket::new(e.entity()).kick_login_failed("Login start invalid at this time"));
                continue;
            };

            let     private_key    = Redacted::from(Rsa::generate(2048).unwrap());
            let     public_key_der = Redacted::from(unsafe { private_key.as_ref() }.public_key_to_der().unwrap());
            let mut verify_token   = [0u8; 4];
            rand::rng().fill_bytes(&mut verify_token);

            ew_packet.write(SendPacket::new(e.entity()).with(S2CLoginEncryptRequestPacket {
                server_id       : r_options.server_id.clone(),
                public_key      : Redacted::from(Cow::Owned(unsafe { public_key_der.as_ref() }.clone())),
                verify_token,
                mojauth_enabled : r_options.mojauth_enabled
            }));

            *flow = PeerLoginFlow::KeyExchange {
                declared_username : username.clone(),
                private_key,
                public_key_der,
                verify_token
            };

        }
    }
}
