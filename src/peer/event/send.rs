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


#[derive(Event)]
pub struct SendPacket {
    entity       : Entity,
    bytes_status : Option<Box<[u8]>>,
    bytes_login  : Option<Box<[u8]>>,
    bytes_config : Option<Box<[u8]>>,
    bytes_play   : Option<Box<[u8]>>,
    switch_state : Option<PacketState>,
    kick         : Option<bool>
}

impl SendPacket {

    #[inline(always)]
    pub fn entity(&self) -> Entity { self.entity }

    #[inline]
    pub fn bytes(&self, state : PacketState) -> Option<&[u8]> {
        match (state) {
            PacketState::Handshake => None,
            PacketState::Status    => self.bytes_status.as_ref().map(|b| &**b),
            PacketState::Login     => self.bytes_login.as_ref().map(|b| &**b),
            PacketState::Config    => self.bytes_config.as_ref().map(|b| &**b),
            PacketState::Play      => self.bytes_play.as_ref().map(|b| &**b)
        }
    }

    #[inline]
    pub unsafe fn bytes_mut(&mut self, state : PacketState) -> Option<&mut [u8]> {
        match (state) {
            PacketState::Handshake => None,
            PacketState::Status    => self.bytes_status.as_mut().map(|b| &mut**b),
            PacketState::Login     => self.bytes_login.as_mut().map(|b| &mut**b),
            PacketState::Config    => self.bytes_config.as_mut().map(|b| &mut**b),
            PacketState::Play      => self.bytes_play.as_mut().map(|b| &mut**b)
        }
    }

    #[inline]
    pub fn take_bytes(mut self, state : PacketState) -> Result<Box<[u8]>, Self> {
        match (state) {
            PacketState::Handshake => Err(self),
            PacketState::Status    => self.bytes_status.take().ok_or(self),
            PacketState::Login     => self.bytes_login.take().ok_or(self),
            PacketState::Config    => self.bytes_config.take().ok_or(self),
            PacketState::Play      => self.bytes_play.take().ok_or(self)
        }
    }

    #[inline]
    pub fn take_bytes_as_vec(self, state : PacketState) -> Result<Vec<u8>, Self> {
        self.take_bytes(state).map(|b| b.into_vec())
    }

    #[inline(always)]
    pub fn has_state(&self, state : PacketState) -> bool { self.bytes(state).is_some() }

    #[inline(always)]
    pub fn switch_state(&self) -> Option<PacketState> { self.switch_state }

    #[inline(always)]
    pub fn is_kick(&self) -> bool { self.kick.is_some_and(|k| k) }

}

impl SendPacket {

    pub fn new(entity : Entity) -> Self { Self {
        entity,
        bytes_status : None,
        bytes_config : None,
        bytes_login  : None,
        bytes_play   : None,
        switch_state : None,
        kick         : None
    } }

}

impl PacketSender for &mut SendPacket {

    #[track_caller]
    fn with_nochange<'l, T>(self, packet : T) -> Self
    where
        T : Into<S2CPackets<'l>>
    {
        let     packet            = packet.into();
        let     (state, _, kick,) = packet.meta();
        let mut buf               = EncodeBuf::new_len_prefixed(packet.encode_prefixed_len());
        unsafe { packet.encode_prefixed(&mut buf); }
        let slot = match (state) {
            PacketState::Handshake => { panic!("can not send handshake packet in event"); },
            PacketState::Status    => &mut self.bytes_status,
            PacketState::Config    => &mut self.bytes_config,
            PacketState::Login     => &mut self.bytes_login,
            PacketState::Play      => &mut self.bytes_play
        };
        if (slot.is_some()) {
            panic!("already added {state:?} packet to event");
        }
        *slot = Some(unsafe { buf.into_inner() });
        if (self.kick.is_some_and(|k| k != kick)) {
            panic!("can not combine kick and non-kick packets in event");
        }
        self.kick = Some(kick);
        self
    }

    #[track_caller]
    fn with<'l, T>(self, packet : T) -> Self
    where
        T : Into<S2CPackets<'l>>
    {
        let packet         = packet.into();
        let (state, _, _,) = packet.meta();
        if let Some(already_state) = self.switch_state {
            panic!("already switching state to {already_state:?} in event");
        }
        self.switch_state = Some(state);
        self.with_nochange(packet)
    }

}


impl PacketSender for SendPacket {

    fn with<'l, T>(mut self, packet : T) -> Self
    where
        T : Into<S2CPackets<'l>>
    {
        (&mut self).with(packet);
        self
    }

    fn with_nochange<'l, T>(mut self, packet : T) -> Self
    where
        T : Into<S2CPackets<'l>>
    {
        (&mut self).with_nochange(packet);
        self
    }

}
