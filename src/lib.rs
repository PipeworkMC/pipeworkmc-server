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


/// Commonly used types.
pub mod prelude {

    bevy_app::plugin_group! {
        #[derive(Debug)]
        pub struct DefaultPlugins {
            bevy_app:::ScheduleRunnerPlugin,
            bevy_time:::TimePlugin,
            crate::peer:::PeerManagerPlugin,
            crate::game::status:::AutoStatusPlugin,
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
        status::{
            AutoStatusPlugin,
            StatusRequest,
            Status, StatusVersion, StatusPlayers, StatusPlayer
        },
        login::{
            AutoApproveLoginsPlugin,
            PlayerLoginRequest,
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
    pub use pipeworkmc_packet as packet;

    /// Bevy
    pub mod bevy {
        pub use bevy_app as app;
        pub use bevy_callback as callback;
        #[expect(missing_docs)]
        pub mod ecs {
            pub use bevy_ecs::*;
            #[expect(missing_docs)]
            pub mod query {
                pub use bevy_ecs::query::*;
                pub use bevy_eqchanged::EqChanged;
            }
        }
        pub use bevy_pareventwriter as pareventwriter;
        pub use bevy_tasks as tasks;
        pub use bevy_time as time;
        /// Bevy prelude.
        pub mod prelude {
            pub use bevy_app::prelude::*;
            pub use bevy_callback::prelude::*;
            pub use bevy_ecs::prelude::*;
            pub use bevy_eqchanged::EqChanged;
            pub use bevy_pareventwriter::ParallelEventWriter;
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
