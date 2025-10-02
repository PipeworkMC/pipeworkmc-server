#![allow(missing_docs)]


use pipeworkmc_server::prelude::*;
use core::time::Duration;
use bevy::{
    prelude::*,
    app::ScheduleRunnerPlugin,
};


fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ScheduleRunnerPlugin::run_loop(Duration::from_millis(1)))
            .set(AutoStatusPlugin {
                motd : "#".dark_red().obfuscate() + " " + ("Hello".red().underline() + " " + "World!".orange().italic()).bold() + " " + "#".dark_red().obfuscate(),
                ..default()
            })
        )
        .add_systems(Update, login)
        .add_systems(Update, logout)
        .run()
}


fn login(
    mut mr_loggedin : MessageReader<PlayerLoggedInMessage>
) {
    for m in mr_loggedin.read() {
        println!("Player {} {} ({}) logged in.", m.peer, m.username, m.uuid);
    }
}

fn logout(
    mut mr_loggedout : MessageReader<PlayerLoggedOutMessage>
) {
    for m in mr_loggedout.read() {
        println!("Player {} {} ({}) logged out.", m.peer, m.username, m.uuid);
    }
}
