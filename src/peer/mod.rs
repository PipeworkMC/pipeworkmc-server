use pipeworkmc_data::{
    bounded_string::BoundedString,
    client_info::ClientInfo
};
use core::net::SocketAddr;
use bevy_ecs::{
    bundle::Bundle,
    component::Component,
    resource::Resource
};


pub mod plugin;

mod reader;
pub mod writer;
pub mod state;

pub mod event;

mod flow;
mod keepalive;


#[derive(Resource)]
pub struct PeerOptions {
    pub server_id          : BoundedString<20>,
    pub server_brand       : String,
    pub compress_threshold : Option<u32>,
    pub mojauth_enabled    : bool
}


#[derive(Bundle)]
struct PeerBundle {
    address    : PeerAddress,
    reader     : reader::PeerStreamReader,
    writer     : writer::PeerStreamWriter,
    state      : state::PeerState,
    login_flow : flow::login::PeerLoginFlow,
    keep_alive : keepalive::PeerKeepAlive,
    info       : ClientInfo
}


#[derive(Component)]
pub struct PeerAddress {
    addr : SocketAddr
}

impl PeerAddress {
    #[inline(always)]
    pub fn addr(&self) -> SocketAddr { self.addr }
}

impl From<SocketAddr> for PeerAddress {
    #[inline(always)]
    fn from(addr : SocketAddr) -> Self { Self { addr } }
}
