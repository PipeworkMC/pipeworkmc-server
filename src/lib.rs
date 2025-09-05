#![feature(

    // Language
    const_precise_live_drops,

    // Syntax
    decl_macro,
    never_type,

    // Standard library
    maybe_uninit_array_assume_init,
    mpmc_channel

)]


pub mod conn;
pub mod game;

pub mod data;
mod util;

pub mod ecs;


pub mod prelude {

    bevy_app::plugin_group! {
        #[derive(Debug)]
        pub struct DefaultPlugins {
            bevy_app:::ScheduleRunnerPlugin,
            crate::conn:::ConnListenerPlugin
        }
    }

    pub use crate::conn::{
        ConnListenerPlugin,
        protocol::Protocol
    };

    pub use crate::game::player::LoggedInEvent;

    pub use crate::data::{
        block_pos::{ BlockPos, DimBlockPos },
        character::CharacterId,
        colour::{ Rgb, Argb },
        game_mode::GameMode,
        ident::Ident,
        itemstack::ItemStack,
        profile::AccountProfile,
        text::{ Text, TextFormatted as _ }
    };
    pub use uuid::Uuid;

    pub mod packet {
        pub use crate::conn::{
            peer::{
                ConnPeerSender,
                event::{
                    IncomingPacketEvent as _,
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
        pub mod prelude {
            pub use crate::ecs::ParallelEventWriter;
            pub use bevy_app::prelude::*;
            pub use bevy_ecs::prelude::*;
            pub use bevy_tasks::prelude::*;
        }
    }

    #[inline(always)]
    pub fn default<D>() -> D
    where
        D : Default
    { D::default() }

}
