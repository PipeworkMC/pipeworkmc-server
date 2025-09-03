use crate::conn::{
    ConnOptions,
    peer::{
        ConnPeerSender,
        ConnPeerState,
        event::login::LoggedInEvent
    },
    protocol::{
        packet::s2c::config::{
            custom_payload::S2CConfigCustomPayloadPacket,
            finish::S2CConfigFinishPacket
        },
        value::channel_data::ChannelData
    }
};
use std::borrow::Cow;
use bevy_ecs::{
    event::EventReader,
    system::{ Query, Res }
};


pub(in crate::conn) fn send_registries(
    mut q_peers     : Query<(&mut ConnPeerSender, &mut ConnPeerState,)>,
    mut er_loggedin : EventReader<LoggedInEvent>,
        r_options   : Res<ConnOptions>
) {
    for event in er_loggedin.read() {
        if let Ok((mut sender, mut state,)) = q_peers.get_mut(event.peer()) {

            sender.send(S2CConfigCustomPayloadPacket { data : ChannelData::Brand {
                brand : Cow::Borrowed(&r_options.server_brand)
            } });

            // TODO: Send registries

            sender.send(S2CConfigFinishPacket);
            unsafe { state.config_finish(); }

        }
    }
}
