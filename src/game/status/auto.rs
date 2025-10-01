use super::StatusRequest;
use crate::peer::Peer;
use crate::game::character::player::{
    PlayerCharacter,
    ReadyPlayerCharacter
};
use pipeworkmc_data::{
    client_info::ClientInfo,
    colour::Rgb,
    profile::AccountProfile,
    text::{
        Text,
        TextComponent,
        TextContent,
        TextStyle
    },
    version::Version
};
use pipeworkmc_packet::s2c::status::response::{
    Status, StatusVersion, StatusPlayers, StatusPlayer
};
use std::borrow::Cow;
use bevy_app::{ App, Plugin };
use bevy_callback::{ AppExt, Req };
use bevy_ecs::{
    resource::Resource,
    query::With,
    system::{ Query, Res }
};


/// A [`Plugin`] which automatically responds to status requests.
pub struct AutoStatusPlugin {
    /// The maximum number of players to display.
    pub max_players : u32,
    /// The MOTD message to display
    pub motd        : Text,
    /// The icon to display.
    pub favicon     : Cow<'static, str>
}

const DEFAULT_MOTD : Text = Text { components : Cow::Borrowed(&[TextComponent {
    content : TextContent::Literal { text : Cow::Borrowed("A Minecraft Server") },
    style   : TextStyle {
        colour : Rgb::GREY,
        ..TextStyle::EMPTY
    }
}]) };

impl Default for AutoStatusPlugin {
    fn default() -> Self {
        Self {
            max_players : 20,
            motd        : DEFAULT_MOTD,
            favicon     : Cow::Borrowed("")
        }
    }
}


#[derive(Resource)]
struct AutoStatusOptions {
    version     : Version,
    max_players : u32,
    motd        : Text,
    favicon     : Cow<'static, str>
}

impl Plugin for AutoStatusPlugin {
    fn build(&self, app : &mut App) {
        app
            .insert_resource(AutoStatusOptions {
                version     : Version::CURRENT,
                motd        : self.motd.clone(),
                max_players : self.max_players,
                favicon     : self.favicon.clone()
            })
            .add_callback(status_response)
        ;
    }
}


fn status_response(
    _req      : Req<StatusRequest>,
    q_players : Query<(&AccountProfile, Option<&ClientInfo>), (With<Peer>, With<PlayerCharacter>, With<ReadyPlayerCharacter>,)>,
    r_options : Res<AutoStatusOptions>
) -> Status<'static> {
    Status {
        version : StatusVersion {
            name     : Cow::Borrowed(r_options.version.latest_name()),
            protocol : r_options.version.id()
        },
        players : Some(StatusPlayers {
            current : q_players.iter().len() as u32,
            max     : r_options.max_players,
            sample  : Cow::Owned(q_players
                .iter().filter_map(|(profile, ci,)| {
                    ci.is_some_and(|ci| ci.allow_motd_listing)
                        .then(|| StatusPlayer { uuid : profile.uuid, name : profile.username.to_string() })
                }).collect::<Vec<_>>()),
        }),
        motd                  : Some(r_options.motd.clone()),
        favicon               : Some(r_options.favicon.clone()),
        requires_chat_signing : false,
        prevents_chat_reports : true
    }
}
