use pipeworkmc_data::{
    bounded_string::BoundedString,
    profile::AccountProfile,
    redacted::Redacted
};
use bevy_ecs::component::Component;
use bevy_tasks::Task;
use openssl::{
    pkey::Private,
    rsa::Rsa
};


pub(in crate::peer) mod start;
pub(in crate::peer) mod encrypt;
pub(in crate::peer) mod mojauth;
pub(in crate::peer) mod approve;


#[derive(Component, Default)]
pub(in crate::peer) enum LoginFlow {

    // The login process has not begun.
    // Server is waiting for peer to send a `C2SLoginStartPacket`.
    #[default]
    Unstarted,

    // Server has begun the key exchange process.
    // Server is waiting for peer to send a `C2SLoginEncryptResponsePacket`.
    KeyExchange {
        declared_username : BoundedString<16>,
        private_key       : Redacted<Rsa<Private>>,
        public_key_der    : Redacted<Vec<u8>>,
        verify_token      : [u8; 4]
    },

    // Key exchange complete.
    // Server is requesting session validation from Mojauth servers.
    Mojauth {
        task : Task<surf::Result<AccountProfile>>
    },

    // Session validated.
    // Server is deciding whether to approve or reject profile.
    Approval,

    // Server has approved the profile.
    // Server is waiting for peer to send a `C2SLoginFinishAcknowledgedPacket`.
    Acknowledge,

    // Login flow is complete.
    // This `Component` is queued for removal.
    Done

}
