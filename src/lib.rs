#![doc = include_str!("../README.md")]


#![feature(

    // Language
    never_type,

    // Standard library
    map_try_insert,
    mpmc_channel

)]


pub use pipeworkmc_data as data;
pub use pipeworkmc_packet as packet;

pub mod peer;
pub mod game;

mod util;

pub mod ecs;


/// Commonly used types.
pub mod prelude {

    bevy_app::plugin_group! {
        #[derive(Debug)]
        pub struct DefaultPlugins {
            bevy_app:::ScheduleRunnerPlugin,
            bevy_time:::TimePlugin,
            crate::peer:::PeerManagerPlugin,
            crate::game::login:::AutoApproveLoginsPlugin
        }
    }

    pub use crate::peer::{
        PeerManagerPlugin,
        PacketSender as _,
        PacketReceived,
        SendPacket
    };
    pub use crate::game::{
        character::player::{
            PlayerCharacterBundle,
            PlayerCharacter
        },
        login::{
            AutoApproveLoginsPlugin,
            PlayerRequestLoginEvent,
            PlayerApproveLoginEvent,
            PlayerLoggedInEvent,
            PlayerLoggedOutEvent
        }
    };

    pub use pipeworkmc_data::{ self as data,
        block_pos::{ BlockPos, DimBlockPos },
        character::CharacterId,
        colour::{ Rgb, Argb },
        game_mode::GameMode,
        ident::Ident,
        item_stack::ItemStack,
        profile::AccountProfile,
        text::{ Text, TextFormatted as _ },
        uuid::Uuid
    };
    pub use pipeworkmc_packet::{ self as packet,
        s2c::status::response::{
            Status, StatusVersion, StatusPlayers, StatusPlayer
        }
    };

    /// Bevy
    pub mod bevy {
        pub use bevy_app as app;
        pub use bevy_ecs as ecs;
        pub use bevy_tasks as tasks;
        pub use bevy_time as time;
        /// Bevy prelude.
        pub mod prelude {
            pub use crate::ecs::ParallelEventWriter;
            pub use bevy_app::prelude::*;
            pub use bevy_ecs::prelude::*;
            pub use bevy_tasks::prelude::*;
            pub use bevy_time::prelude::*;
        }
    }
    pub use bevy_app::PluginGroup as _;

    /// Shorthand for [`Default::default`].
    #[inline]
    pub fn default<D>() -> D
    where
        D : Default
    { D::default() }

}
