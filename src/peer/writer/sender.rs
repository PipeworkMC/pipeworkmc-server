use crate::peer::{
    PeerAddress,
    writer::PeerStreamWriter,
    event::SendPacket
};
use pipeworkmc_codec::meta::PacketState;
use pipeworkmc_data::text::{
    Text,
    TextComponent,
    TextContent
};
use pipeworkmc_packet::s2c::{
    S2CPackets,
    login::disconnect::S2CLoginDisconnectPacket,
    config::disconnect::S2CConfigDisconnectPacket,
    play::disconnect::S2CPlayDisconnectPacket
};
use std::borrow::Cow;
use bevy_ecs::{
    event::EventReader,
    query::With,
    system::Query
};


pub(in crate::peer) fn handle_send_events(
    mut q_peers   : Query<(&mut PeerStreamWriter,), (With<PeerAddress>,)>,
    mut er_packet : EventReader<SendPacket>
) {
    for e in er_packet.read() {
        if let Ok((mut writer,)) = q_peers.get_mut(e.entity()) {
            writer.handle_send_packet(e);
        }
    }
}


pub trait PacketSender
where
    Self : Sized
{

    fn with_before_switch<'l, T>(self, packet : T) -> Self
    where
        T : Into<S2CPackets<'l>>;

    fn with<'l, T>(self, packet : T) -> Self
    where
        T : Into<S2CPackets<'l>>;

    fn with_switch_state(self, state : PacketState, skip_intermediate : bool) -> Self;

    #[track_caller]
    fn kick<'l, S>(self, reason : S) -> Self
    where
        S : Into<&'l Text>
    {
        let reason = reason.into();
        println!("{reason}");
        self
            .with_before_switch(S2CLoginDisconnectPacket::from(reason))
            .with_before_switch(S2CConfigDisconnectPacket::from(reason))
            .with_before_switch(S2CPlayDisconnectPacket::from(reason))
    }

    #[track_caller]
    #[inline]
    fn kick_generic(self) -> Self {
        self.kick(&Text { components : Cow::Borrowed(&[ TextComponent { content : TextContent::Translate {
            key : Cow::Borrowed("multiplayer.disconnect.generic"), fallback : None, with : Cow::Borrowed(&[])
        }, ..TextComponent::EMPTY } ]) })
    }

    #[track_caller]
    #[inline]
    fn kick_end_of_stream(self) -> Self {
        self.kick(&Text { components : Cow::Borrowed(&[ TextComponent { content : TextContent::Translate {
            key : Cow::Borrowed("disconnect.endOfStream"), fallback : None, with : Cow::Borrowed(&[])
        }, ..TextComponent::EMPTY } ]) })
    }

    #[track_caller]
    #[inline]
    fn kick_packet_error<S>(self, message : S) -> Self
    where
        S : Into<Text>
    {
        self.kick(&(Text { components : Cow::Borrowed(&[
            TextComponent { content : TextContent::Translate {
                key : Cow::Borrowed("disconnect.packetError"), fallback : None, with : Cow::Borrowed(&[])
            }, ..TextComponent::EMPTY },
            TextComponent { content : TextContent::Literal {
                text : Cow::Borrowed(": ")
            }, ..TextComponent::EMPTY }
        ]) } + message))
    }

    #[track_caller]
    #[inline]
    fn kick_login_failed<S>(self, message : S) -> Self
    where
        S : Into<Text>
    {
        self.kick(&Text { components : Cow::Owned(vec![ TextComponent { content : TextContent::Translate {
            key : Cow::Borrowed("disconnect.loginFailedInfo"), fallback : None, with : Cow::Owned(vec![message.into()])
        }, ..TextComponent::EMPTY } ]) })
    }

    #[track_caller]
    #[inline]
    fn kick_timeout(self) -> Self {
        self.kick(&Text { components : Cow::Borrowed(&[ TextComponent { content : TextContent::Translate {
            key : Cow::Borrowed("disconnect.timeout"), fallback : None, with : Cow::Borrowed(&[])
        }, ..TextComponent::EMPTY } ]) })
    }

    #[track_caller]
    #[inline]
    fn kick_duplicate_login(self) -> Self {
        self.kick(&Text { components : Cow::Borrowed(&[ TextComponent { content : TextContent::Translate {
            key : Cow::Borrowed("multiplayer.disconnect.duplicate_login"), fallback : None, with : Cow::Borrowed(&[])
        }, ..TextComponent::EMPTY } ]) })
    }

    #[track_caller]
    #[inline]
    fn kick_name_taken(self) -> Self {
        self.kick(&Text { components : Cow::Borrowed(&[ TextComponent { content : TextContent::Translate {
            key : Cow::Borrowed("multiplayer.disconnect.name_taken"), fallback : None, with : Cow::Borrowed(&[])
        }, ..TextComponent::EMPTY } ]) })
    }

}
