#![feature(

    // Language
    never_type,

    // Standard library
    mpmc_channel

)]


pub mod conn;
pub mod game;
pub use pipeworkmc_data as data;

mod util;

pub mod ecs;


pub mod prelude {

    bevy_app::plugin_group! {
        #[derive(Debug)]
        pub struct DefaultPlugins {
            bevy_app:::ScheduleRunnerPlugin,
            bevy_time:::TimePlugin,
            crate::conn:::ConnListenerPlugin,
            crate::game::player::login:::AutoApproveLoginsPlugin
        }
    }

    pub use crate::conn::{
        ConnListenerPlugin,
        protocol::Protocol
    };

    pub use crate::game::player::login::{
        PlayerRequestLoginEvent,
        PlayerApproveLoginEvent,
        PlayerLoggedInEvent
    };

    pub use crate::data::{
        block_pos::{ BlockPos, DimBlockPos },
        character::CharacterId,
        colour::{ Rgb, Argb },
        game_mode::GameMode,
        ident::Ident,
        itemstack::ItemStack,
        profile::AccountProfile,
        text::{ Text, TextFormatted as _ },
        uuid::Uuid
    };

    pub mod packet {
        pub use crate::conn::{
            peer::{
                ConnPeerSender,
                event::{
                    handshake::IncomingHandshakePacketEvent,
                    status::IncomingStatusPacketEvent,
                    login::IncomingLoginPacketEvent,
                    config::IncomingConfigPacketEvent,
                    play::IncomingPlayPacketEvent
                }
            },
            protocol::packet::{
                c2s::{ self,
                    C2SPackets,
                    handshake::C2SHandshakePackets,
                    status::C2SStatusPackets,
                    login::C2SLoginPackets,
                    config::C2SConfigPackets,
                    play::C2SPlayPackets
                },
                s2c::{ self,
                    S2CPackets,
                    status::{
                        S2CStatusPackets,
                        response::{
                            Status,
                            StatusVersion,
                            StatusPlayers,
                            StatusPlayer
                        }
                    },
                }
            }
        };
    }

    pub mod bevy {
        pub use bevy_app as app;
        pub use bevy_ecs as ecs;
        pub use bevy_tasks as tasks;
        pub use bevy_time as time;
        pub mod prelude {
            pub use crate::ecs::ParallelEventWriter;
            pub use bevy_app::prelude::*;
            pub use bevy_ecs::prelude::*;
            pub use bevy_tasks::prelude::*;
            pub use bevy_time::prelude::*;
        }
    }

    #[inline(always)]
    pub fn default<D>() -> D
    where
        D : Default
    { D::default() }

}
