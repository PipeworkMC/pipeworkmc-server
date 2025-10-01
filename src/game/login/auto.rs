use crate::game::login::PlayerLoginRequest;
use pipeworkmc_data::{
    profile::AccountProfile,
    text::{
        Text,
        TextComponent,
        TextContent
    }
};
use std::borrow::Cow;
use bevy_app::{ App, Plugin };
use bevy_callback::{ AppExt, Req };
use bevy_ecs::{
    entity::Entity,
    system::Query
};


/// A [`Plugin`] which automatically handles duplicate logins on the server.
#[derive(Default)]
pub struct AutoApproveLoginsPlugin {
    /// How to handle duplicate logins on the server.
    pub duplicate_strategy : DuplicateStrategy
}

/// How to handle duplicate logins on the server.
#[derive(Default)]
pub enum DuplicateStrategy {
    /// Allow duplicate UUIDs.
    Allow,
    /// When a new player with the same UUID as an existing player joins, kick the older connection.
    #[default]
    KickOld,
    /// When a new player with the same UUID as an existing player joins, kick the newer connection.
    RejectNew
}


impl Plugin for AutoApproveLoginsPlugin {
    fn build(&self, app : &mut App) {
        match (self.duplicate_strategy) {
            DuplicateStrategy::Allow     => { app.add_callback(allow_duplicates); },
            DuplicateStrategy::KickOld   => { app.add_callback(kick_old_duplicates); },
            DuplicateStrategy::RejectNew => { app.add_callback(reject_new_duplicates); }
        }
    }
}


/// Unconditionally approves login requests.
fn allow_duplicates(
    _ : Req<PlayerLoginRequest>
) -> Result<(), Text> { Ok(()) }


const DUPLICATE_LOGIN : Text = Text { components : Cow::Borrowed(&[ TextComponent { content : TextContent::Translate {
    key : Cow::Borrowed("multiplayer.disconnect.duplicate_login"), fallback : None, with : Cow::Borrowed(&[])
}, ..TextComponent::EMPTY } ]) };

/// Approves login requests, kicking any existing logins with the same UUID.
fn kick_old_duplicates(
    req     : Req<PlayerLoginRequest>,
    q_peers : Query<(Entity, &AccountProfile,)>
) -> Result<(), Text> {
    for (entity, profile,) in &q_peers {
        if (req.peer != entity && req.uuid != profile.uuid) {
            return Err(DUPLICATE_LOGIN);
        }
    }
    Ok(())
}


const NAME_TAKEN : Text = Text { components : Cow::Borrowed(&[ TextComponent { content : TextContent::Translate {
    key : Cow::Borrowed("multiplayer.disconnect.name_taken"), fallback : None, with : Cow::Borrowed(&[])
}, ..TextComponent::EMPTY } ]) };

/// Approves login requests, as long as no existing logins have the same UUID.
fn reject_new_duplicates(
    req     : Req<PlayerLoginRequest>,
    q_peers : Query<(Entity, &AccountProfile,)>
) -> Result<(), Text> {
    if (q_peers.iter().any(|(entity, profile,)| req.peer != entity && req.uuid == profile.uuid)) {
        return Err(NAME_TAKEN);
    }
    Ok(())
}
