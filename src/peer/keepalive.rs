use crate::peer::{
    Peer,
    writer::PacketSender,
    message::{
        SendPacket,
        PacketReceived
    }
};
use pipeworkmc_codec::meta::PacketBound;
use pipeworkmc_packet::{
    c2s::{
        C2SPackets,
        config::{
            C2SConfigPackets,
            keep_alive::C2SConfigKeepAlivePacket
        },
        play::{
            C2SPlayPackets,
            keep_alive::C2SPlayKeepAlivePacket
        }
    },
    s2c::{
        config::keep_alive::S2CConfigKeepAlivePacket,
        play::keep_alive::S2CPlayKeepAlivePacket
    }
};
use core::time::Duration;
use bevy_ecs::{
    component::Component,
    entity::Entity,
    message::MessageReader,
    query::With,
    system::Query
};
use bevy_parmessagewriter::ParallelMessageWriter;
use bevy_time::{
    Timer,
    TimerMode
};


#[derive(Component)]
pub(in crate::peer) struct PeerKeepAlive {
    next_bound : PacketBound,
    timer      : Timer,
    id         : u64
}
impl Default for PeerKeepAlive {
    fn default() -> Self { Self {
        next_bound : PacketBound::S2C,
        timer      : Timer::new(Duration::ZERO, TimerMode::Once),
        id         : 0
    } }
}
impl PeerKeepAlive {

    fn sent_s2c(&mut self) {
        self.next_bound = PacketBound::C2S;
        self.timer.set_duration(Duration::from_secs(10));
    }

    fn received_c2s(&mut self) {
        self.next_bound = PacketBound::S2C;
        self.timer.set_duration(Duration::from_millis(2500));
        self.id = self.id.wrapping_add(1);
    }

}


pub(in crate::peer) fn handle_keepalive_expiration(
    mut q_peers   : Query<(Entity, &mut PeerKeepAlive,), (With<Peer>,)>,
        mw_packet : ParallelMessageWriter<SendPacket>
) {
    q_peers.par_iter_mut().for_each(|(entity, mut keepalive,)| {
        if (keepalive.timer.is_finished()) {
            match (keepalive.next_bound) {

                PacketBound::C2S => {
                    mw_packet.write(SendPacket::new(entity).kick_timeout());
                },

                PacketBound::S2C => {
                    mw_packet.write(SendPacket::new(entity)
                        .with_before_switch(S2CConfigKeepAlivePacket { transaction : keepalive.id })
                        .with_before_switch(S2CPlayKeepAlivePacket   { transaction : keepalive.id })
                    );
                    keepalive.sent_s2c();
                }

            }
        }
    });
}


pub(in crate::peer) fn handle_keepalive_response(
    mut q_peers   : Query<(&mut PeerKeepAlive,), (With<Peer>,)>,
    mut mr_packet : MessageReader<PacketReceived>
) {
    for m in mr_packet.read() {
        if
            let C2SPackets::Config(C2SConfigPackets::KeepAlive(C2SConfigKeepAlivePacket { .. }))
                | C2SPackets::Play(C2SPlayPackets::KeepAlive(C2SPlayKeepAlivePacket { .. }))
                = &m.packet
            && let Ok((mut keepalive,)) = q_peers.get_mut(m.peer)
        { keepalive.received_c2s(); }
    }
}
