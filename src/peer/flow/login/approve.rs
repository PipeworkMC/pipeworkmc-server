use super::LoginFlow;
use crate::peer::{
    writer::PacketSender,
    state::PeerState,
    event::{
        PacketReceived,
        SendPacket
    }
};
use crate::game::player::login::{
    PlayerApproveLoginEvent
};
use pipeworkmc_data::profile::AccountProfile;
use pipeworkmc_packet::{
    c2s::{
        C2SPackets,
        login::{
            C2SLoginPackets,
            finish_acknowledged::C2SLoginFinishAcknowledgedPacket
        }
    },
    s2c::login::finish::S2CLoginFinishPacket
};
use bevy_ecs::{
    event::{
        EventReader,
        EventWriter
    },
    system::{
        Commands,
        Query
    }
};


pub(in crate::peer) fn alert_approved_logins(
    mut q_peers   : Query<(&mut PeerState, &AccountProfile, &mut LoginFlow,)>,
    mut er_login  : EventReader<PlayerApproveLoginEvent>,
    mut ew_packet : EventWriter<SendPacket>
) {
    for e in er_login.read() {
        if let Ok((mut state, profile, mut flow)) = q_peers.get_mut(e.entity()) {
            let LoginFlow::Approval = &*flow else {
                continue;
            };

            ew_packet.write(SendPacket::new(e.entity()).with(S2CLoginFinishPacket {
                profile : AccountProfile {
                    uuid     : profile.uuid,
                    username : profile.username.clone(),
                    skin     : None
                }
            }));
            unsafe { state.login_finish(); }
            *flow = LoginFlow::Acknowledge;
        }
    }
}


pub(in crate::peer) fn handle_login_acknowledge(
    mut cmds      : Commands,
    mut q_peers   : Query<(&mut PeerState, &mut LoginFlow,)>,
    mut er_packet : EventReader<PacketReceived>,
    mut ew_packet : EventWriter<SendPacket>
) {
    for e in er_packet.read() {
        if let C2SPackets::Login(C2SLoginPackets::FinishAcknowledged(
            C2SLoginFinishAcknowledgedPacket { }
        )) = e.packet() {
            if let Ok((mut state, mut flow,)) = q_peers.get_mut(e.entity()) {
                let LoginFlow::Acknowledge = &*flow else {
                    ew_packet.write(SendPacket::new(e.entity()).kick_login_failed("Login acknowledgement invalid at this time"));
                    continue;
                };

                unsafe { state.login_finish_acknowledged(); }
                *flow = LoginFlow::Done;
                cmds.entity(e.entity()).remove::<LoginFlow>();
            }
        }
    }
}
