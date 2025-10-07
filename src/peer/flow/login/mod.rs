use pipeworkmc_data::{
    bounded_string::BoundedString,
    redacted::Redacted
};
use bevy_ecs::component::Component;
use openssl::{
    pkey::Private,
    rsa::Rsa
};


pub(in crate::peer) mod start;
pub(in crate::peer) mod encrypt;
#[cfg(feature = "mojauth")]
pub(in crate::peer) mod mojauth;
pub(in crate::peer) mod finish;


#[derive(Component, Default)]
pub(in crate::peer) enum PeerLoginFlow {

    // The login process has not begun.
    // Server is waiting for peer to send a `C2SLoginStartPacket`.
    #[default]
    Unstarted,

    // Server has begun the key exchange process.
    // Server is waiting for peer to send a `C2SLoginEncryptResponsePacket`.
    KeyExchange {
        declared_username : BoundedString<16>,
        private_key       : Redacted<Rsa<Private>>,
        #[cfg(feature = "mojauth")]
        public_key_der    : Redacted<Vec<u8>>,
        verify_token      : [u8; 4]
    },

    // Key exchange complete.
    // Server is requesting session validation from Mojauth servers.
    #[cfg(feature = "mojauth")]
    Mojauth {
        task : bevy_tasks::Task<surf::Result<pipeworkmc_data::profile::AccountProfile>>
    },

    // Server has approved the profile.
    // Server is waiting for peer to send a `C2SLoginFinishAcknowledgedPacket`.
    Acknowledge,

    // Login flow is complete or cancelled.
    // This `Component` is queued for removal.
    Done

}
