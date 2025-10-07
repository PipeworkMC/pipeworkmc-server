//! Peer data and systems.


use pipeworkmc_data::{
    bounded_string::BoundedString,
    client_info::ClientInfo,
    redacted::Redacted
};
use core::net::SocketAddr;
use bevy_ecs::{
    bundle::Bundle,
    component::Component,
    resource::Resource
};


mod plugin;
pub use plugin::PeerManagerPlugin;

mod reader;
mod writer;
pub use writer::PacketSender;
mod state;
pub use state::PeerState;

mod message;
pub use message::{
    PacketReceived,
    SendPacket
};

mod flow;
mod keepalive;


/// Global peer options.
#[derive(Resource)]
pub struct PeerOptions {

    /// The server ID to use in the hash when authenticating.
    pub server_id          : BoundedString<20>,

    /// The server brand which is shown in the client F3 debug screen.
    ///
    /// *Note: Changing this will not automatically update the brand known by existing peers. They will need to rejoin.*
    pub server_brand       : String,

    /// How large packets need to be before being compressed.
    ///
    /// `None` to disable packet compression.
    ///
    /// *Note: Changing this will not automatically update the compresson threshold of existing peers. They will need to rejoin.*
    pub compress_threshold : Option<u32>,

    /// Whether the Mojang authentication servers should be contacted to
    ///  confirm player's identities on join.
    ///
    /// **WARNING**
    /// *Changing this will not automatically update the authentication requirement of existing peers. They will need to rejoin.*
    /// *This may allow multiple players with the same username to be online concurrently.*
    ///
    /// **WARNING**
    /// Setting this to `false` allows any player to join with any username,
    ///  potentially letting them steal other player's data.
    #[cfg(feature = "mojauth")]
    pub mojauth_enabled    : bool

}


#[derive(Bundle)]
struct PeerBundle {
    peer       : Peer,
    reader     : reader::PeerStreamReader,
    writer     : writer::PeerStreamWriter,
    state      : state::PeerState,
    login_flow : flow::login::PeerLoginFlow,
    keep_alive : keepalive::PeerKeepAlive,
    info       : ClientInfo
}


/// A marker for all peers.
#[derive(Component)]
#[component(immutable)]
pub struct Peer {
    /// Remote connection address.
    pub remote_addr : Redacted<SocketAddr>
}
