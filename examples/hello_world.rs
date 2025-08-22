use pipeworkmc_server::conn::{
    ConnListenerPlugin,
    peer::event::IncomingHandshakePacketEvent
};
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
    mut er_handshake : EventReader<IncomingHandshakePacketEvent>
) {
    er_handshake.par_read().for_each(|event| {
        println!("{:?}", event);
    });
}
