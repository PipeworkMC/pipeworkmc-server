#![feature(

    // Language
    decl_macro,
    never_type,

    // Standard library
    maybe_uninit_array_assume_init,
    mpmc_channel

)]


pub mod conn;

pub mod util;


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
        peer::event::IncomingPacketEvent as _,
        protocol::Protocol
    };

    pub use crate::conn::protocol::value::{
        colour::{ Rgb, Argb },
        ident::Ident,
        itemstack::ItemStack,
        text::{ Text, TextFormatted as _ },
        varint::VarInt
    };
    pub use uuid::Uuid;

    pub mod packet {
        pub use crate::conn::{
            peer::event::{
                status::StatusRequestEvent,
                OutgoingPacketEvent
            },
            protocol::packet::{
                c2s::{
                    C2SPackets,
                    handshake::C2SHandshakePackets,
                    status::C2SStatusPackets,
                    login::C2SLoginPackets
                },
                s2c::{
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
            pub use crate::util::par_eventwriter::ParallelEventWriter;
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
