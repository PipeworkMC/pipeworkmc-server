use crate::peer::{
    Peer,
    PacketSender,
    SendPacket
};
use crate::game::character::Character;
use pipeworkmc_data::character::{
    CharacterId,
    CharacterPos,
    CharacterRot,
    CharacterVel
};
use pipeworkmc_packet::s2c::play::{
    add_character::S2CPlayAddCharacterPacket,
    remove_characters::S2CPlayRemoveCharactersPacket
};
use std::{
    borrow::Cow,
    collections::{ HashSet, HashMap }
};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    event::EventWriter,
    query::{ With, Changed },
    removal_detection::RemovedComponents,
    system::{ Local, Query }
};


/// A [`Component`] which tracks what players are able to see this character when in range.
#[derive(Component, Default)]
pub struct CharacterVisibility {
    should_be_visible_to  : HashSet<Entity>,
    modified_visibilities : HashSet<Entity>
}

impl CharacterVisibility {

    /// Show this character to a player when in range.
    pub fn show_to(&mut self, player : Entity) {
        if (self.should_be_visible_to.insert(player)) {
            self.modified_visibilities.insert(player);
        }
    }

    /// Hides this character from a player.
    pub fn hide_from(&mut self, player : Entity) {
        if (self.should_be_visible_to.remove(&player)) {
            self.modified_visibilities.insert(player);
        }
    }

    /// Hides this character from all players.
    pub fn hide_from_all(&mut self) {
        self.modified_visibilities.extend(&self.should_be_visible_to);
        self.should_be_visible_to.clear();
    }

}

impl CharacterVisibility {

    /// Returns whether this character will be visible to a player if in range.
    #[inline]
    pub fn should_be_visible_to(&self, player : Entity) -> bool {
        self.should_be_visible_to.contains(&player)
    }

}


/// A [`Component`] which tracks what characters this player is currently able to see.
#[derive(Component, Default)]
pub(crate) struct VisibleCharacters {
    visible_characters : HashMap<Entity, CharacterId>
}


/// Updates character visibilities for players when the [`CharacterVisibility`] [`Component`] is modified.
pub(super) fn update_visibilities(
    mut q_peers      : Query<(&mut VisibleCharacters,), (With<Peer>,)>,
    mut q_vis        : Query<(
        Entity,
        &mut CharacterVisibility,
        &CharacterId,
        &Character,
        &CharacterPos,
        &CharacterRot,
        &CharacterVel,
    ), (Changed<CharacterVisibility>,)>,
    mut ew_packet    : EventWriter<SendPacket>
) {

    for (
            character_entity,
        mut character_visibility,
            &character_id,
            character,
            &pos, &rot, &vel,
    ) in &mut q_vis {
        for &peer_entity in &character_visibility.modified_visibilities {
            if let Ok((mut visible_characters,)) = q_peers.get_mut(peer_entity) {
                // If the player is supposed to be able to see this character, but doesn't, send an add character packet.
                if (character_visibility.should_be_visible_to.contains(&peer_entity)) { // TODO: Make sure the player is in range.
                    if (visible_characters.visible_characters.try_insert(character_entity, character_id).is_ok()) {
                        ew_packet.write(SendPacket::new(peer_entity).with(
                            S2CPlayAddCharacterPacket {
                                eid  : character_id,
                                uuid : character.uuid,
                                ty   : character.ty,
                                pos, rot,
                                data : character.data,
                                vel
                            }
                        ));
                    }
                }
                // If the player is not supposed to be able to see this character, but can, send a remove character packet.
                else if let Some(character_id) = visible_characters.visible_characters.remove(&character_entity) {
                    ew_packet.write(SendPacket::new(peer_entity).with(
                        S2CPlayRemoveCharactersPacket { eids : Cow::Borrowed(&[character_id]) }
                    ));
                }
            }
        }
        character_visibility.modified_visibilities.clear();
    }
}


/// Updates character visibilities for players when a [`CharacterVisibility`] [`Component`] is removed.
pub(super) fn on_remove_character(
    mut q_peers   : Query<(Entity, &mut VisibleCharacters,), (With<Peer>,)>,
    mut d_vis     : RemovedComponents<CharacterVisibility>,
    mut ew_packet : EventWriter<SendPacket>,
    mut l_buf0    : Local<Vec<Entity>>,
    mut l_buf1    : Local<Vec<CharacterId>>
) {
    // Get the list of characters that are to be removed.
    l_buf0.clear();
    l_buf0.extend(d_vis.read());

    for (peer_entity, mut visible_characters,) in &mut q_peers {
        // For each player, get which characters that are to be removed are visible.
        l_buf1.clear();
        for character_entity in &*l_buf0 {
            if let Some(character_id) = visible_characters.visible_characters.remove(character_entity) {
                l_buf1.push(character_id);
            }
        }
        // Send a remove character packet for the characters that are to be removed.
        if (! l_buf1.is_empty()) {
            ew_packet.write(SendPacket::new(peer_entity).with(
                S2CPlayRemoveCharactersPacket { eids : Cow::Borrowed(l_buf1.as_slice()) }
            ));
        }
    }
}
