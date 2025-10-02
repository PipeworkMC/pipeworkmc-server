use crate::peer::{
    Peer,
    writer::PeerStreamWriter,
    message::SendPacket
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
    message::MessageReader,
    query::With,
    system::Query
};


pub(in crate::peer) fn handle_send_messages(
    mut q_peers   : Query<(&mut PeerStreamWriter,), (With<Peer>,)>,
    mut mr_packet : MessageReader<SendPacket>
) {
    for m in mr_packet.read() {
        if let Ok((mut writer,)) = q_peers.get_mut(m.entity()) {
            writer.handle_send_packet(m);
        }
    }
}


/// A type which can be used to send packets to peers.
pub trait PacketSender
where
    Self : Sized
{

    /// Sends a packet without switching states.
    ///
    /// One packet for each state can be added.
    ///
    /// If the outgoing state of the peer does not match the state of this packet, it is not sent.
    fn with_before_switch<'l, T>(self, packet : T) -> Self
    where
        T : Into<S2CPackets<'l>>;

    /// Sends a packet after switching state if needed.
    fn with<'l, T>(self, packet : T) -> Self
    where
        T : Into<S2CPackets<'l>>;

    /// Switches the peer to the given state.
    fn with_switch_state(self, state : PacketState, skip_intermediate : bool) -> Self;

    /// Sends a kick packet to the peer.
    #[track_caller]
    fn kick<'l, S>(self, reason : S) -> Self
    where
        S : Into<&'l Text>
    {
        let reason = reason.into();
        self
            .with_before_switch(S2CLoginDisconnectPacket::from(reason))
            .with_before_switch(S2CConfigDisconnectPacket::from(reason))
            .with_before_switch(S2CPlayDisconnectPacket::from(reason))
    }

    /// Sends a generic disconnect kick packet to the peer.
    #[track_caller]
    #[inline]
    fn kick_generic(self) -> Self {
        self.kick(&Text { components : Cow::Borrowed(&[ TextComponent { content : TextContent::Translate {
            key : Cow::Borrowed("multiplayer.disconnect.generic"), fallback : None, with : Cow::Borrowed(&[])
        }, ..TextComponent::EMPTY } ]) })
    }

    /// Sends an end of stream kick packet to the peer.
    #[track_caller]
    #[inline]
    fn kick_end_of_stream(self) -> Self {
        self.kick(&Text { components : Cow::Borrowed(&[ TextComponent { content : TextContent::Translate {
            key : Cow::Borrowed("disconnect.endOfStream"), fallback : None, with : Cow::Borrowed(&[])
        }, ..TextComponent::EMPTY } ]) })
    }

    /// Sends a network protocol error kick packet to the peer.
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

    /// Sends a login error kick packet to the peer.
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

    /// Sends a timeout kick packet to the peer.
    #[track_caller]
    #[inline]
    fn kick_timeout(self) -> Self {
        self.kick(&Text { components : Cow::Borrowed(&[ TextComponent { content : TextContent::Translate {
            key : Cow::Borrowed("disconnect.timeout"), fallback : None, with : Cow::Borrowed(&[])
        }, ..TextComponent::EMPTY } ]) })
    }

}
