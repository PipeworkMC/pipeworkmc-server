use crate::peer::{
    event::SendPacket,
    writer::PacketSender,
    PeerAddress
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


#[derive(Component, Default)]
pub struct CharacterVisibility {
    should_be_visible_to  : HashSet<Entity>,
    modified_visibilities : HashSet<Entity>
}

impl CharacterVisibility {

    pub fn show_to(&mut self, player : Entity) {
        if (self.should_be_visible_to.insert(player)) {
            self.modified_visibilities.insert(player);
        }
    }

    pub fn hide_from(&mut self, player : Entity) {
        if (self.should_be_visible_to.remove(&player)) {
            self.modified_visibilities.insert(player);
        }
    }

    pub fn hide_from_all(&mut self) {
        self.modified_visibilities.extend(&self.should_be_visible_to);
        self.should_be_visible_to.clear();
    }

}

impl CharacterVisibility {

    #[inline(always)]
    pub fn is_visible_to(&self, player : Entity) -> bool {
        self.should_be_visible_to.contains(&player)
    }

}


pub(super) fn update_visibilities(
    mut q_peers      : Query<(&mut VisibleCharacters,), (With<PeerAddress>,)>,
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
                if (character_visibility.should_be_visible_to.contains(&peer_entity)) {
                    if let Ok(_) = visible_characters.visible_characters.try_insert(character_entity, character_id) {
                        ew_packet.write(SendPacket::new(peer_entity).with(
                            S2CPlayAddCharacterPacket {
                                eid  : character_id,
                                uuid : character.uuid(),
                                ty   : character.ty(),
                                pos, rot,
                                data : character.data(),
                                vel
                            }
                        ));
                        // TODO: Send add entity packet.
                    }
                } else {
                    if let Some(character_id) = visible_characters.visible_characters.remove(&character_entity) {
                        ew_packet.write(SendPacket::new(peer_entity).with(
                            S2CPlayRemoveCharactersPacket { eids : Cow::Borrowed(&[character_id]) }
                        ));
                    }
                }
            }
        }
        character_visibility.modified_visibilities.clear();
    }
}


#[derive(Component, Default)]
pub(crate) struct VisibleCharacters {
    visible_characters : HashMap<Entity, CharacterId>
}

pub(super) fn on_remove_character(
    mut q_peers   : Query<(Entity, &mut VisibleCharacters,), (With<PeerAddress>,)>,
    mut d_vis     : RemovedComponents<CharacterVisibility>,
    mut ew_packet : EventWriter<SendPacket>,
    mut l_buf0    : Local<Vec<Entity>>,
    mut l_buf1    : Local<Vec<CharacterId>>
) {
    l_buf0.clear();
    l_buf0.extend(d_vis.read());
    for (peer_entity, mut visible_characters,) in &mut q_peers {
        l_buf1.clear();
        for character_entity in &*l_buf0 {
            if let Some(character_id) = visible_characters.visible_characters.remove(character_entity) {
                l_buf1.push(character_id);
            }
        }
        if (! l_buf1.is_empty()) {
            ew_packet.write(SendPacket::new(peer_entity).with(
                S2CPlayRemoveCharactersPacket { eids : Cow::Borrowed(l_buf1.as_slice()) }
            ));
        }
    }
}
