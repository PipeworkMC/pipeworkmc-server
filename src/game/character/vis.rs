use crate::peer::{
    Peer,
    PacketSender,
    SendPacket
};
use crate::game::character::{
    Character,
    player::ViewDist
};
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
    collections::{
        HashSet, HashMap,
        hash_map::Entry as HashMapEntry
    }
};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    lifecycle::RemovedComponents,
    message::MessageWriter,
    query::{ With, Changed },
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
#[derive(Component)]
pub(crate) struct VisibleCharacters {
    this_entity            : Entity,
    visible_character_ids  : HashMap<Entity, CharacterId>,
    next_character_id      : u32, // TODO: Make NonZeroU32
    reusable_character_ids : HashSet<u32> // TODO: Make NonZeroU32
}
impl VisibleCharacters {

    #[inline]
    pub(crate) fn new(this_entity : Entity) -> Self { Self {
        this_entity,
        visible_character_ids  : HashMap::new(),
        next_character_id      : 0,
        reusable_character_ids : HashSet::new()
    } }

    fn try_insert(&mut self, entity : Entity) -> Option<CharacterId> {
        match (self.visible_character_ids.entry(entity)) {
            HashMapEntry::Occupied(_) => None,
            HashMapEntry::Vacant(entry) => {
                // TODO: Use 0 only when entity == self.this_entity.
                let chid = CharacterId(self.reusable_character_ids.iter().next().cloned()
                    .map_or_else(|| {
                        let chid = self.next_character_id;
                        self.next_character_id += 1;
                        chid
                    }, |chid| {
                        self.reusable_character_ids.remove(&chid);
                        chid
                    })
                );
                entry.insert(chid);
                Some(chid)
            }
        }
    }

    fn remove(&mut self, entity : Entity) -> Option<CharacterId> {
        self.visible_character_ids.remove(&entity).map(|character_id| {
            self.reusable_character_ids.insert(character_id.0);
            character_id
        })
    }

}


/// Updates character visibilities for players when the [`CharacterVisibility`] [`Component`] is modified.
pub(super) fn update_visibilities(
    mut q_peers   : Query<(
        &mut VisibleCharacters,
        &CharacterPos,
        &ViewDist
    ), (With<Peer>,)>,
    mut q_vis     : Query<(
        Entity,
        &mut CharacterVisibility,
        &Character,
        &CharacterPos,
        &CharacterRot,
        &CharacterVel,
    ), (Changed<CharacterVisibility>,)>,
    mut mw_packet : MessageWriter<SendPacket>
) {

    for (
            character_entity,
        mut character_visibility,
            character,
            &character_pos,
            &character_rot,
            &character_vel,
    ) in &mut q_vis {
        for &peer_entity in &character_visibility.modified_visibilities {
            if let Ok((mut visible_characters, player_pos, player_view_dist)) = q_peers.get_mut(peer_entity) {
                // If the player is supposed to be able to see this character, but doesn't, send an add character packet.
                if (character_visibility.should_be_visible_to.contains(&peer_entity)
                    // Also make sure the character is in range.
                    && character_pos.chunk().cardinal_dist(player_pos.chunk()) < (player_view_dist.as_u8() as u32)
                ) {
                    if let Some(character_id) = visible_characters.try_insert(character_entity) {
                        mw_packet.write(SendPacket::new(peer_entity).with(
                            S2CPlayAddCharacterPacket {
                                eid  : character_id,
                                uuid : character.uuid,
                                ty   : character.ty,
                                pos  : character_pos,
                                rot  : character_rot,
                                data : character.data,
                                vel  : character_vel
                            }
                        ));
                    }
                }
                // If the player is not supposed to be able to see this character, but can, send a remove character packet.
                else if let Some(character_id) = visible_characters.remove(character_entity) {
                    mw_packet.write(SendPacket::new(peer_entity).with(
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
    mut d_vis     : RemovedComponents<CharacterVisibility>, // TODO: Use observer.
    mut mw_packet : MessageWriter<SendPacket>,
    mut l_buf0    : Local<Vec<Entity>>,
    mut l_buf1    : Local<Vec<CharacterId>>
) {
    // Get the list of characters that are to be removed.
    l_buf0.clear();
    l_buf0.extend(d_vis.read());

    for (peer_entity, mut visible_characters,) in &mut q_peers {
        // For each player, get which characters that are to be removed are visible.
        l_buf1.clear();
        for &character_entity in &*l_buf0 {
            if let Some(character_id) = visible_characters.remove(character_entity) {
                l_buf1.push(character_id);
            }
        }
        // Send a remove character packet for the characters that are to be removed.
        if (! l_buf1.is_empty()) {
            mw_packet.write(SendPacket::new(peer_entity).with(
                S2CPlayRemoveCharactersPacket { eids : Cow::Borrowed(l_buf1.as_slice()) }
            ));
        }
    }
}
