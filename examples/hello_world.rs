use pipeworkmc_server::{
    PROTOCOL,
    conn::{
        ConnListenerPlugin,
        peer::event::{
            status::IncomingStatusPacketEvent,
            OutgoingPacketEvent
        },
        protocol::{
            packet::{
                c2s::status::{
                    C2SStatusPackets,
                    request::C2SStatusRequestPacket
                },
                s2c::status::response::{
                    S2CStatusResponsePacket,
                    Status,
                    StatusVersion
                }
            },
            value::text::TextFormatted
        }
    }
};
use pipeworkmc_server::util::par_eventwriter::ParallelEventWriter;
use core::time::Duration;
use bevy_app::{
    App, AppExit,
    ScheduleRunnerPlugin,
    Update
};
use bevy_ecs::event::EventReader;


fn main() -> AppExit {
    App::new()
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::ZERO))
        .add_plugins(ConnListenerPlugin::default())
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
                    protocol : PROTOCOL
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
