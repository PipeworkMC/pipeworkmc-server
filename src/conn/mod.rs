use crate::conn::protocol::value::client_info::ClientInfo;
use core::net::{
    SocketAddr,
    SocketAddrV4,
    Ipv4Addr
};
use std::{
    borrow::Cow,
    io,
    net::{ TcpListener, ToSocketAddrs },
    sync::Arc
};
use bevy_app::{ App, Plugin, Update };
use bevy_ecs::{
    resource::Resource,
    schedule::IntoScheduleConfigs,
    system::{ Commands, Res }
};
use bevy_tasks::{ IoTaskPool, TaskPool };


pub mod peer;
use peer::{
    ConnPeerBundle,
    ConnPeer,
    ConnPeerReader,
    ConnPeerIncoming,
    ConnPeerDecoder,
    ConnPeerWriter,
    ConnPeerSender,
    ConnPeerState,
    event::login::ConnPeerLoginFlow
};

pub mod protocol;
use protocol::value::bounded_string::BoundedString;


/// Enables the connection listener on install.
#[derive(Clone)]
pub struct ConnListenerPlugin {

    /// Addresses to listen on.
    ///
    /// The default port the game uses is `25565`.
    pub listen_addrs       : Cow<'static, [SocketAddr]>,

    pub server_id          : BoundedString<20>,

    pub server_brand       : String,

    /// How large packets need to be before being compressed.
    ///
    /// `None` to disable packet compression.
    pub compress_threshold : Option<u32>,

    /// Whether the Mojang authentication servers should be contacted to
    ///  confirm player's identities on join.
    ///
    /// **WARNING**
    /// Setting this to `false` allows any player to join with any username,
    ///  potentially letting them steal other player's data.
    pub mojauth_enabled    : bool

}


impl ConnListenerPlugin {
    const DEFAULT_LISTEN_ADDRS : &[SocketAddr] = &[
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 25565))
    ];
}


impl Default for ConnListenerPlugin {
    fn default() -> Self {
        Self {
            listen_addrs       : Cow::Borrowed(Self::DEFAULT_LISTEN_ADDRS),
            server_id          : BoundedString::try_from("PipeworkMC").unwrap(),
            server_brand       : String::from("PipeworkMC"),
            compress_threshold : Some(64),
            mojauth_enabled    : false
        }
    }
}


impl Plugin for ConnListenerPlugin {
    fn build(&self, app : &mut App) {
        _ = IoTaskPool::get_or_init(TaskPool::new);
        app .add_event::<peer::event::handshake::IncomingHandshakePacketEvent>()
            .add_event::<peer::event::status::IncomingStatusPacketEvent>()
            .add_event::<peer::event::login::IncomingLoginPacketEvent>()
            .add_event::<peer::event::login::LoggedInEvent>()
            .add_event::<peer::event::config::IncomingConfigPacketEvent>()
            .insert_resource(ConnListener::new(&*self.listen_addrs).unwrap()) // TODO: Error handler.
            .insert_resource(ConnOptions {
                server_id          : self.server_id.clone(),
                server_brand       : self.server_brand.clone(),
                compress_threshold : self.compress_threshold,
                mojauth_enabled    : self.mojauth_enabled
            })
            .add_systems(Update, accept_conn_peers)
            .add_systems(Update, peer::read_conn_peer_incoming)
            .add_systems(Update, peer::decode_conn_peer_incoming)
            .add_systems(Update, peer::write_conn_peer_outgoing)
            .add_systems(Update, peer::time_out_conns)
            .add_systems(Update, peer::event::handshake::handle_intention.before(peer::decode_conn_peer_incoming))
            .add_systems(Update, peer::event::status::respond_to_pings)
            .add_systems(Update, peer::event::login::handle_login_flow.before(peer::decode_conn_peer_incoming))
            .add_systems(Update, peer::event::login::poll_mojauths_tasks)
            .add_systems(Update, peer::event::config::send_registries)
            .add_systems(Update, peer::event::config::handle_config.before(peer::decode_conn_peer_incoming))
        ;
    }
}



#[derive(Resource)]
pub struct ConnOptions {
    pub server_id          : BoundedString<20>,
    pub server_brand       : String,
    pub compress_threshold : Option<u32>,
    pub mojauth_enabled    : bool
}



#[derive(Resource)]
struct ConnListener(TcpListener);

impl ConnListener {
    pub fn new<A>(addr : A) -> io::Result<Self>
    where
        A : ToSocketAddrs
    {
        let listener = TcpListener::bind(addr)?;
        listener.set_nonblocking(true)?;
        Ok(Self(listener))
    }
}


fn accept_conn_peers(
    mut cmds       : Commands,
        r_listener : Res<ConnListener>
) {
    match (r_listener.0.accept()) {
        Ok((write_stream, addr,)) => {
            write_stream.set_nodelay(true).unwrap(); // TODO: Error handler.
            write_stream.set_nonblocking(true).unwrap(); // TODO: Error handler.
            let read_stream = write_stream.try_clone().unwrap(); // TODO: Error handler.
            let state = ConnPeerState::handshake();
            cmds.spawn(ConnPeerBundle {
                peer       : ConnPeer::from(addr),
                reader     : ConnPeerReader::from(read_stream),
                incoming   : ConnPeerIncoming::default(),
                decoder    : ConnPeerDecoder::default(),
                writer     : ConnPeerWriter::from(write_stream),
                sender     : ConnPeerSender::from(Arc::clone(state.outgoing_state_arc())),
                state,
                login_flow : ConnPeerLoginFlow::default(),
                info       : ClientInfo::default()
            });
        },
        Err(err) if (err.kind() == io::ErrorKind::WouldBlock) => { },
        Err(err) => panic!("{err}") // TODO: Error handler.
    }
}
