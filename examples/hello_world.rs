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
        )
        .add_systems(Update, login)
        .add_systems(Update, logout)
        .run()
}


fn login(
    mut er_loggedin : EventReader<PlayerLoggedInEvent>
) {
    for e in er_loggedin.read() {
        println!("Player {} {} ({}) logged in.", e.peer, e.username, e.uuid);
    }
}

fn logout(
    mut er_loggedout : EventReader<PlayerLoggedOutEvent>
) {
    for e in er_loggedout.read() {
        println!("Player {} {} ({}) logged out.", e.peer, e.username, e.uuid);
    }
}
