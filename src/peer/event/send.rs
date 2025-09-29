use crate::peer::writer::PacketSender;
use pipeworkmc_codec::{
    encode::{
        PrefixedPacketEncode,
        EncodeBuf
    },
    meta::PacketState
};
use pipeworkmc_packet::s2c::S2CPackets;
use bevy_ecs::{
    entity::Entity,
    event::Event
};


/// An [`Event`] which can be emitted to send a packet to a peer.
#[derive(Event)]
pub struct SendPacket {
    entity        : Entity,
    status_before : Option<Box<[u8]>>,
    login_before  : Option<Box<[u8]>>,
    config_before : Option<Box<[u8]>>,
    play_before   : Option<Box<[u8]>>,
    switch_state  : Option<(PacketState, Option<Box<[u8]>>, bool,)>,
    kick          : Option<bool>,
    #[cfg(debug_assertions)]
    sent_by       : &'static core::panic::Location<'static>
}

impl SendPacket {

    /// The [`Entity`] of the peer to send the packet to.
    #[inline]
    pub fn entity(&self) -> Entity { self.entity }

    /// Bytes of the packet to send before switching state.
    pub fn before_switch(&self, state : PacketState) -> Option<&[u8]> {
        match (state) {
            PacketState::Handshake => None,
            PacketState::Status    => self.status_before.as_deref(),
            PacketState::Login     => self.login_before.as_deref(),
            PacketState::Config    => self.config_before.as_deref(),
            PacketState::Play      => self.play_before.as_deref()
        }
    }

    /// State and bytes of the packet to send after switching state.
    #[inline]
    pub fn after_switch(&self) -> Option<(PacketState, Option<&[u8]>, bool,)> {
        self.switch_state.as_ref().map(|(new_state, b, skip_intermediate,)| (*new_state, b.as_ref().map(|b| &**b), *skip_intermediate,))
    }

    /// Whether this is a packet which will kick the peer.
    #[inline]
    pub fn is_kick(&self) -> bool { self.kick.is_some_and(|k| k) }

    /// The [`Location`](core::panic:Location) where this [`SendPacket`] was created.
    ///
    /// Only available with `#[cfg(debug_assertions)]`.
    #[cfg(debug_assertions)]
    #[inline]
    pub fn sent_by(&self) -> &'static core::panic::Location<'static> { self.sent_by }

}

impl SendPacket {

    /// Create a new [`SendPacket`] event which can be sent to a peer.
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn new(entity : Entity) -> Self { Self {
        entity,
        status_before : None,
        login_before  : None,
        config_before : None,
        play_before   : None,
        switch_state  : None,
        kick          : None,
        #[cfg(debug_assertions)]
        sent_by       : core::panic::Location::caller()
    } }

}

impl PacketSender for &mut SendPacket {

    #[track_caller]
    fn with_before_switch<'l, T>(self, packet : T) -> Self
    where
        T : Into<S2CPackets<'l>>
    {
        let     packet            = packet.into();
        let     (state, _, kick,) = packet.meta();

        let slot = match (state) {
            PacketState::Handshake => { panic!("can not send handshake packet in event"); },
            PacketState::Status    => &mut self.status_before,
            PacketState::Login     => &mut self.login_before,
            PacketState::Config    => &mut self.config_before,
            PacketState::Play      => &mut self.play_before
        };
        if (slot.is_some()) {
            panic!("already added {state:?} before switch packet to event");
        }

        if (self.kick.is_some_and(|k| k != kick)) {
            panic!("can not combine kick and non-kick packets in event");
        }
        self.kick = Some(kick);

        let mut buf = EncodeBuf::new_len_prefixed(packet.encode_prefixed_len());
        // SAFETY: `buf` has space for `packet.encode_prefixed_len()` bytes.
        unsafe { packet.encode_prefixed(&mut buf); }
        // SAFETY: `packet.encode_prefixed()` is required to fill the entire buffer.
        *slot = Some(unsafe { buf.into_inner() });

        self
    }

    #[track_caller]
    fn with<'l, T>(self, packet : T) -> Self
    where
        T : Into<S2CPackets<'l>>
    {
        let packet            = packet.into();
        let (state, _, kick,) = packet.meta();

        if let Some(switch_state) = &self.switch_state {
            panic!("already switching state to {switch_state:?}");
        }

        if (self.kick.is_some_and(|k| k != kick)) {
            panic!("can not combine kick and non-kick packets in event");
        }
        self.kick = Some(kick);

        let mut buf = EncodeBuf::new_len_prefixed(packet.encode_prefixed_len());
        unsafe { packet.encode_prefixed(&mut buf); }
        self.switch_state = Some((state, Some(unsafe { buf.into_inner() }), false,));

        self
    }

    #[track_caller]
    fn with_switch_state(self, state : PacketState, skip_intermediate : bool) -> Self {
        if let Some(switch_state) = &self.switch_state {
            panic!("already switching state to {switch_state:?}");
        }
        self.switch_state = Some((state, None, skip_intermediate));

        self
    }

}


impl PacketSender for SendPacket {

    #[track_caller]
    #[inline]
    fn with_before_switch<'l, T>(mut self, packet : T) -> Self
    where
        T : Into<S2CPackets<'l>>
    {
        (&mut self).with_before_switch(packet);
        self
    }

    #[track_caller]
    #[inline]
    fn with<'l, T>(mut self, packet : T) -> Self
    where
        T : Into<S2CPackets<'l>>
    {
        (&mut self).with(packet);
        self
    }

    #[track_caller]
    #[inline]
    fn with_switch_state(mut self, state : PacketState, skip_intermediate : bool) -> Self {
        (&mut self).with_switch_state(state, skip_intermediate);
        self
    }

}
