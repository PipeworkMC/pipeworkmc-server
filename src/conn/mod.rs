use core::net::{
    SocketAddr,
    SocketAddrV4,
    Ipv4Addr
};
use std::{
    borrow::Cow,
    io,
    net::{ TcpListener, ToSocketAddrs }
};
use bevy_app::{ App, Plugin, Update };
use bevy_ecs::{
    resource::Resource,
    schedule::IntoScheduleConfigs,
    system::{ Commands, Res }
};


pub mod peer;
use peer::{
    ConnPeerBundle,
    ConnPeer,
    ConnPeerReader,
    ConnPeerIncoming,
    ConnPeerDecoder,
    ConnPeerState,
    // ConnPeerWriter
};

pub mod protocol;


/// Enables the connection listener on install.
#[derive(Clone)]
pub struct ConnListenerPlugin {

    /// Addresses to listen on.
    ///
    /// The default port the game uses is `25565`.
    pub listen_addrs       : Cow<'static, [SocketAddr]>,

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
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 25565)),
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 25565))
    ];
}


impl Default for ConnListenerPlugin {
    fn default() -> Self {
        Self {
            listen_addrs       : Cow::Borrowed(Self::DEFAULT_LISTEN_ADDRS),
            compress_threshold : Some(64),
            mojauth_enabled    : true
        }
    }
}


impl Plugin for ConnListenerPlugin {
    fn build(&self, app : &mut App) {
        app .add_event::<peer::event::IncomingHandshakePacketEvent>()
            .add_event::<peer::event::IncomingStatusPacketEvent>()
            .insert_resource(ConnListener::new(&*self.listen_addrs).unwrap()) // TODO: Error handler.
            .insert_resource(ConnCompressThreshold(self.compress_threshold))
            .insert_resource(ConnMojauthEnabled(self.mojauth_enabled))
            .add_systems(Update, accept_conn_peers)
            .add_systems(Update, peer::read_conn_peer_incoming)
            .add_systems(Update, peer::decode_conn_peer_incoming)
            .add_systems(Update, peer::event::switch_handshake_state.before(peer::decode_conn_peer_incoming))
        ;
    }
}



#[derive(Resource)]
struct ConnCompressThreshold(#[expect(dead_code)] Option<u32>);

#[derive(Resource)]
struct ConnMojauthEnabled(#[expect(dead_code)] bool);



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
            cmds.spawn(ConnPeerBundle {
                peer     : ConnPeer::from(addr),
                reader   : ConnPeerReader { stream : read_stream },
                incoming : ConnPeerIncoming::default(),
                decoder  : ConnPeerDecoder::default(),
                // writer   : ConnPeerWriter { stream : write_stream },
                state    : ConnPeerState::handshake()
            });
        },
        Err(err) if (err.kind() == io::ErrorKind::WouldBlock) => { },
        Err(err) => panic!("{err}") // TODO: Error handler.
    }
}
