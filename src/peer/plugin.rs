use crate::peer::{
    PeerOptions,
    PeerAddress,
    PeerBundle,
    reader::{ self, PeerStreamReader },
    writer::{ self, PeerStreamWriter },
    state::{ self,
        PeerState,
        KEEPALIVE_TIMEOUT
    },
    event,
    flow::{ self,
        login::LoginFlow
    }
};
use crate::game::player::login::{
    PlayerRequestLoginEvent,
    PlayerApproveLoginEvent,
    PlayerLoggedInEvent
};
use pipeworkmc_codec::meta::{
    AtomicPacketState,
    PacketState
};
use pipeworkmc_data::{
    bounded_string::BoundedString,
    character::NextCharacterId,
    client_info::ClientInfo
};
use core::{
    net::{
        SocketAddr,
        SocketAddrV4,
        Ipv4Addr
    },
    sync::atomic::AtomicBool
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
use bevy_time::common_conditions::on_timer;


/// Enables the connection listener and peer manager on install.
#[derive(Clone)]
pub struct PeerManagerPlugin {

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


impl PeerManagerPlugin {
    const DEFAULT_LISTEN_ADDRS : &[SocketAddr] = &[
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 25565))
    ];
}


impl Default for PeerManagerPlugin {
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


impl Plugin for PeerManagerPlugin {
    fn build(&self, app : &mut App) {
        _ = IoTaskPool::get_or_init(TaskPool::new);

        app .add_event::<event::PacketReceived>()
            .add_event::<event::SendPacket>()
            .add_event::<PlayerRequestLoginEvent>()
            .add_event::<PlayerApproveLoginEvent>()
            .add_event::<PlayerLoggedInEvent>()

            .insert_resource(PeerListener::new(&*self.listen_addrs).unwrap()) // TODO: Error handler.
            .insert_resource(PeerOptions {
                server_id          : self.server_id.clone(),
                server_brand       : self.server_brand.clone(),
                compress_threshold : self.compress_threshold,
                mojauth_enabled    : self.mojauth_enabled
            })
            .insert_resource(NextCharacterId::default())

            .add_systems(Update, accept_new_peers)
            .add_systems(Update, reader::read_peer_bytes)
            .add_systems(Update, reader::decode_peer_packets)
            .add_systems(Update, writer::handle_send_events)
            .add_systems(Update, writer::write_peer_bytes)
            .add_systems(Update, state::timeout_peers)

            .add_systems(Update, flow::handshake::handle_intention
                .before(reader::decode_peer_packets))
            .add_systems(Update, flow::status::respond_to_pings)
            .add_systems(Update, flow::login::start::begin_key_exchange)
            .add_systems(Update, flow::login::encrypt::finish_key_exchange_and_check_mojauth)
            .add_systems(Update, flow::login::mojauth::poll_mojauth_tasks
                .run_if(flow::login::mojauth::is_mojauth_enabled))
            .add_systems(Update, flow::login::approve::alert_approved_logins)
            .add_systems(Update, flow::login::approve::handle_login_acknowledge)

            // .add_systems(Update, peer::event::config::handle_config
            //     .before(peer::decode_conn_peer_incoming))
            // .add_systems(Update, peer::event::play::handle_keepalive
            //     .run_if(on_timer(KEEPALIVE_TIMEOUT / 2)))
        ;
    }
}



#[derive(Resource)]
struct PeerListener(TcpListener);

impl PeerListener {
    pub fn new<A>(addr : A) -> io::Result<Self>
    where
        A : ToSocketAddrs
    {
        let listener = TcpListener::bind(addr)?;
        listener.set_nonblocking(true)?;
        Ok(Self(listener))
    }
}


fn accept_new_peers(
    mut cmds       : Commands,
        r_listener : Res<PeerListener>
) {
    match (r_listener.0.accept()) {
        Ok((write_stream, addr,)) => {
            write_stream.set_nodelay(true).unwrap(); // TODO: Error handler.
            write_stream.set_nonblocking(true).unwrap(); // TODO: Error handler.
            let read_stream    = write_stream.try_clone().unwrap(); // TODO: Error handler.
            let outgoing_state = Arc::new(AtomicPacketState::new(PacketState::Handshake));
            let disconnecting  = Arc::new(AtomicBool::new(false));
            let state          = unsafe { PeerState::new(Arc::clone(&outgoing_state), Arc::clone(&disconnecting)) };
            cmds.spawn(PeerBundle {
                address    : PeerAddress::from(addr),
                reader     : PeerStreamReader::new(read_stream),
                writer     : PeerStreamWriter::new(write_stream, outgoing_state, disconnecting),
                state,
                login_flow : LoginFlow::default(),
                info       : ClientInfo::default()
            });
        },
        Err(err) if (err.kind() == io::ErrorKind::WouldBlock) => { },
        Err(err) => panic!("{err}") // TODO: Error handler.
    }
}
