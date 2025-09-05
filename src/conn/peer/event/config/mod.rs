use crate::conn::{
    peer::{
        ConnPeerState,
        ConnPeerBrand,
        event::IncomingPacketEvent
    },
    protocol::packet::c2s::config::{
        C2SConfigPackets,
        client_info::C2SConfigClientInfoPacket,
        custom_payload::C2SConfigCustomPayloadPacket
    }
};
use crate::data::channel_data::ChannelData;
use std::time::Instant;
use bevy_ecs::{
    entity::Entity,
    event::{ Event, EventReader },
    system::{ ParallelCommands, Query }
};


#[derive(Event, Debug)]
pub struct IncomingConfigPacketEvent {
    peer      : Entity,
    packet    : C2SConfigPackets,
    timestamp : Instant
}

impl IncomingConfigPacketEvent {
    #[inline]
    pub(crate) fn new(peer : Entity, packet : C2SConfigPackets) -> Self {
        Self { peer, packet, timestamp : Instant::now() }
    }
}

impl IncomingPacketEvent for IncomingConfigPacketEvent {
    type Packet = C2SConfigPackets;

    #[inline(always)]
    fn peer(&self) -> Entity { self.peer }

    #[inline(always)]
    fn packet(&self) -> &Self::Packet { &self.packet }
    #[inline(always)]
    fn take_packet(self) -> Self::Packet { self.packet }

    #[inline(always)]
    fn timestamp(&self) -> Instant { self.timestamp }

}


pub(in crate::conn) fn handle_config(
        pcmds     : ParallelCommands,
    mut q_peers   : Query<(Entity, &mut ConnPeerState,)>,
    mut er_config : EventReader<IncomingConfigPacketEvent>
) {
    for event in er_config.read() {
        if let Ok((entity, mut state,)) = q_peers.get_mut(event.peer()) {
            match (event.packet()) {


                C2SConfigPackets::ClientInfo(C2SConfigClientInfoPacket { info }) => { // Handle ClientInfo packet from play state as well.
                    pcmds.command_scope(|mut cmds| { cmds.entity(entity).insert(info.clone()); });
                },


                C2SConfigPackets::CustomPayload(C2SConfigCustomPayloadPacket { data }) => { // Handle CustomPayload packets from all other states as well.
                    if let ChannelData::Brand { brand } = data {
                        pcmds.command_scope(|mut cmds| { cmds.entity(entity).insert(ConnPeerBrand { brand : brand.to_string() }); });
                    }
                },


                C2SConfigPackets::FinishAcknowledged(_) => {
                    unsafe { state.config_finish_acknowledged(); }
                }


            }
        }
    }
}
