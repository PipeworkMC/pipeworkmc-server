//! Player data and systems.


use super::Character;
use crate::peer::{
    Peer,
    PacketSender,
    SendPacket,
    PacketReceived
};
use pipeworkmc_data::{
    character::CharacterType,
    client_info::ClientInfo,
    game_mode::GameMode,
    profile::AccountProfile
};
use pipeworkmc_packet::{
    c2s::{
        C2SPackets,
        config::{
            C2SConfigPackets,
            client_info::C2SConfigClientInfoPacket
        },
        play::{
            C2SPlayPackets,
            client_info::C2SPlayClientInfoPacket
        }
    },
    s2c::play::game_event::S2CPlayGameEventPacket
};
use core::num::NonZeroU8;
use bevy_ecs::{
    bundle::Bundle,
    component::Component,
    entity::Entity,
    lifecycle::Add,
    message::{
        MessageReader,
        MessageWriter
    },
    observer::On,
    query::{ With, Has },
    system::{ Commands, Query }
};
use bevy_eqchanged::EqChanged;


mod dimension;
pub use dimension::*;


/// A [`Bundle`] of [`Component`]s required to create a player character.
#[derive(Bundle, Default)]
pub struct PlayerCharacterBundle {
    /// A marker for player-type characters.
    pub player      : PlayerCharacter,
    /// Client settings.
    pub client_info : ClientInfo,
    /// The dimension that the player is in.
    pub dimension   : Dimension,
    /// The player's view distance.
    pub view_dist   : ViewDist,
    /// The player's game mode.
    pub game_mode   : GameMode,
    /// General character components.
    pub character   : super::CharacterBundle
}


/// A marker for player-type characters.
#[derive(Component, Default)]
#[component(immutable)]
#[require(OldNoRespawnScreen, OldReducedDebugInfo)]
#[non_exhaustive]
pub struct PlayerCharacter;

/// A marker for player-type characters who have finished logging in and should respond to [`Component`] changes.
#[derive(Component, Default)]
#[component(immutable)]
#[non_exhaustive]
pub struct ReadyPlayerCharacter;

/// Iterates through all characters marked as player-type, and sets data used by entity spawners.
pub(in crate::game::character) fn set_character(
        add         : On<Add, PlayerCharacter>,
    mut cmds        : Commands,
        q_character : Query<(Entity, &AccountProfile,)>
) {
    if let Ok((entity, profile,)) = q_character.get(add.entity) {
        cmds.entity(entity).insert(Character {
            ty   : CharacterType::Player,
            uuid : profile.uuid,
            data : 0
        });
    }
}


/// The client's declared brand.
#[derive(Component)]
pub struct ClientBrand {
    /// The brand (Vanilla, fabric, forge, etc).
    pub brand : String
}

/// Whether the player should be logged in with hardcore hearts.
///
/// Can not be changed after logging in.
#[derive(Component)]
#[component(immutable)]
pub struct IsHardcore;


/// The player's view distance.
#[derive(Component)]
pub struct ViewDist(NonZeroU8);
impl ViewDist {
    /// Returns the inner value as a [`NonZeroU8`].
    #[inline]
    pub fn as_n0u8(&self) -> NonZeroU8 { self.0 }
    /// Returns the inner value as a [`u8`].
    #[inline]
    pub fn as_u8(&self) -> u8 { self.0.get() }
}
impl Default for ViewDist {
    #[inline]
    fn default() -> Self {
        // SAFETY: 8 is not 0.
        Self(unsafe { NonZeroU8::new_unchecked(8) })
    }
}


/// Updates client info on received.
pub(in crate::game::character) fn update_client_info(
    mut cmds      : Commands,
    mut mr_packet : MessageReader<PacketReceived>
) {
    for m in mr_packet.read() {
        if let C2SPackets::Config(C2SConfigPackets::ClientInfo(C2SConfigClientInfoPacket { info }))
            | C2SPackets::Play(C2SPlayPackets::ClientInfo(C2SPlayClientInfoPacket { info }))
        = &m.packet {
            cmds.entity(m.peer).insert(info.clone());
        }
    }
}


/// Sends game mode updates to players on change.
pub(in crate::game::character) fn update_game_mode(
        q_players : Query<(Entity, &GameMode), (With<Peer>, With<PlayerCharacter>, With<ReadyPlayerCharacter>, EqChanged<GameMode>,)>,
    mut mw_packet : MessageWriter<SendPacket>
) {
    mw_packet.write_batch(q_players.iter().map(|(entity, &game_mode,)| {
        SendPacket::new(entity).with(S2CPlayGameEventPacket::ChangeGameMode { to : game_mode })
    }));
}


/// Whether the player will immediately respawn without showing the respawn screen on death.
#[derive(Component)]
#[require(OldNoRespawnScreen)]
pub struct NoRespawnScreen;
#[derive(Component, Default)]
pub(in crate::game::character) struct OldNoRespawnScreen(bool);

/// Sends respawn screen updates to players on change.
pub(in crate::game::character) fn update_no_respawn_screen(
    mut q_players : Query<(
        Entity,
        Has<NoRespawnScreen>,
        &mut OldNoRespawnScreen,
        Has<ReadyPlayerCharacter>,
    ), (With<Peer>, With<PlayerCharacter>,)>,
    mut ew_packet : MessageWriter<SendPacket>
) {
    for (entity, nrs, mut old_nrs, is_ready) in &mut q_players {
        if (nrs != old_nrs.0) {
            old_nrs.0 = nrs;
            if (is_ready) {
                ew_packet.write(SendPacket::new(entity).with(
                    S2CPlayGameEventPacket::SetRespawnScreen { show_respawn_screen : true }
                ));
            }
        }
    }
}

/// Whether the player's F3 debug screen is reduced.
#[derive(Component, Default)]
pub struct ReducedDebugInfo; // TODO: Update RDI on changed.
#[derive(Component, Default)]
pub(in crate::game::character) struct OldReducedDebugInfo(#[expect(dead_code)] bool);
