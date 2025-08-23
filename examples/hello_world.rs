use pipeworkmc_server::prelude::*;
use pipeworkmc_server::prelude::packet::*;
use pipeworkmc_server::conn::protocol::packet::{
    c2s::status::request::C2SStatusRequestPacket,
    s2c::status::response::{
        S2CStatusResponsePacket,
        Status,
        StatusVersion
    }
};
use bevy::prelude::*;


fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, server_status_response)
        .run()
}


fn server_status_response(
    mut er_status : EventReader<IncomingStatusPacketEvent>,
        ew_packet : ParallelEventWriter<OutgoingPacketEvent>
) {
    er_status.par_read().for_each(|event| {
        if let C2SStatusPackets::Request(C2SStatusRequestPacket) = event.packet() {
            ew_packet.write(OutgoingPacketEvent::new(event.peer(), S2CStatusResponsePacket::from(Status {
                version               : StatusVersion {
                    name     : Some("Hello, World!".into()),
                    protocol : Protocol::LATEST.id()
                },
                players               : None,
                motd                  : Some(("Hello,".bold().red() + " World!").italic()),
                favicon               : "".into(),
                enforces_secure_chat  : false,
                prevents_chat_reports : true
            })));
        }
    });
}
