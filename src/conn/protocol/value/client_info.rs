use crate::conn::protocol::{
    codec::decode::{
        PacketDecode,
        DecodeBuf,
        IncompleteDecodeError
    },
    value::{
        bounded_string::{
            BoundedString,
            BoundedStringDecodeError
        },
        varint::{
            VarInt,
            VarIntDecodeError
        }
    }
};
use core::fmt::{ self, Debug, Display, Formatter };
use bevy_ecs::component::Component;


#[derive(Clone, Component, Debug)]
pub struct ClientInfo {
    pub locale             : BoundedString<16>,
    pub view_dist          : u8,
    pub chat_mode          : ChatMode,
    pub chat_colours       : bool,
    pub skin_layers        : SkinLayers,
    pub left_handed        : bool,
    pub text_filtered      : bool,
    pub allow_motd_listing : bool,
    pub particle_status    : ParticleStatus
}
impl Default for ClientInfo {
    fn default() -> Self { Self {
        locale             : unsafe { BoundedString::new_unchecked("en_us") },
        view_dist          : 2,
        chat_mode          : ChatMode::Enabled,
        chat_colours       : false,
        skin_layers        : SkinLayers::ALL,
        left_handed        : false,
        text_filtered      : false,
        allow_motd_listing : false,
        particle_status    : ParticleStatus::All
    } }
}


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ChatMode {
    Enabled,
    CommandsOnly,
    Hidden
}


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SkinLayers(u8);
impl SkinLayers {

    pub const ALL  : Self = Self(0b01111111);
    pub const NONE : Self = Self(0b00000000);

    #[inline(always)]
    pub const fn as_byte(&self) -> u8 { self.0 }

    pub const CAPE : u8 = 0b00000001;
    pub const fn cape(&self) -> bool { self.get(Self::CAPE) }
    pub const fn set_cape(&mut self, enabled : bool) { self.set(Self::CAPE, enabled); }

    pub const JACKET : u8 = 0b00000010;
    pub const fn jacket(&self) -> bool { self.get(Self::JACKET) }
    pub const fn set_jacket(&mut self, enabled : bool) { self.set(Self::JACKET, enabled); }

    pub const LEFT_SLEEVE : u8 = 0b00000100;
    pub const fn left_sleeve(&self) -> bool { self.get(Self::LEFT_SLEEVE) }
    pub const fn set_left_sleeve(&mut self, enabled : bool) { self.set(Self::LEFT_SLEEVE, enabled); }

    pub const RIGHT_SLEEVE : u8 = 0b00001000;
    pub const fn right_sleeve(&self) -> bool { self.get(Self::RIGHT_SLEEVE) }
    pub const fn set_right_sleeve(&mut self, enabled : bool) { self.set(Self::RIGHT_SLEEVE, enabled); }

    pub const LEFT_PANTS_LEG : u8 = 0b00010000;
    pub const fn left_pants_leg(&self) -> bool { self.get(Self::LEFT_PANTS_LEG) }
    pub const fn set_left_pants_leg(&mut self, enabled : bool) { self.set(Self::LEFT_PANTS_LEG, enabled); }

    pub const RIGHT_PANTS_LEG : u8 = 0b00100000;
    pub const fn right_pants_leg(&self) -> bool { self.get(Self::RIGHT_PANTS_LEG) }
    pub const fn set_right_pants_leg(&mut self, enabled : bool) { self.set(Self::RIGHT_PANTS_LEG, enabled); }

    pub const HAT : u8 = 0b01000000;
    pub const fn hat(&self) -> bool { self.get(Self::HAT) }
    pub const fn set_hat(&mut self, enabled : bool) { self.set(Self::HAT, enabled); }

    #[inline(always)]
    const fn get(&self, flag : u8) -> bool { (self.0 & flag) != 0 }
    #[inline(always)]
    const fn set(&mut self, flag : u8, enabled : bool) {
        if (enabled) {
            self.0 |= flag;
        } else {
            self.0 &= ! flag;
        }
    }

}
impl Debug for SkinLayers {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("SkinLayers")
            .field("cape", &self.cape())
            .field("jacket", &self.jacket())
            .field("left_sleeve", &self.left_sleeve())
            .field("right_sleeve", &self.right_sleeve())
            .field("left_pants_leg", &self.left_pants_leg())
            .field("right_pants_leg", &self.right_pants_leg())
            .field("hat", &self.hat())
            .finish()
    }
}


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ParticleStatus {
    All,
    Decreased,
    Minimal
}


impl PacketDecode for ClientInfo {
    type Error = ClientInfoDecodeError;

    fn decode(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    { Ok(Self {
        locale             : <_>::decode(buf).map_err(ClientInfoDecodeError::Locale)?,
        view_dist          : <_>::decode(buf).map_err(ClientInfoDecodeError::ViewDist)?,
        chat_mode          : match (*<VarInt<u32>>::decode(buf).map_err(ClientInfoDecodeError::ChatMode)?) {
            0 => ChatMode::Enabled,
            1 => ChatMode::CommandsOnly,
            2 => ChatMode::Hidden,
            v => { return Err(ClientInfoDecodeError::UnknownChatMode(v))? }
        },
        chat_colours       : <_>::decode(buf).map_err(ClientInfoDecodeError::ChatColours)?,
        skin_layers        : SkinLayers(<_>::decode(buf).map_err(ClientInfoDecodeError::SkinFlags)?),
        left_handed        : match (*<VarInt<u32>>::decode(buf).map_err(ClientInfoDecodeError::MainHand)?) {
            0 => true,
            1 => false,
            v => { return Err(ClientInfoDecodeError::UnknownMainHand(v))? }
        },
        text_filtered      : <_>::decode(buf).map_err(ClientInfoDecodeError::TextFiltered)?,
        allow_motd_listing : <_>::decode(buf).map_err(ClientInfoDecodeError::AllowMotdListing)?,
        particle_status    : match (*<VarInt<u32>>::decode(buf).map_err(ClientInfoDecodeError::ParticleStatus)?) {
            0 => ParticleStatus::All,
            1 => ParticleStatus::Decreased,
            2 => ParticleStatus::Minimal,
            v => { return Err(ClientInfoDecodeError::UnknownParticleStatus(v))? }
        },
    }) }
}


#[derive(Debug)]
pub enum ClientInfoDecodeError {
    Locale(BoundedStringDecodeError),
    ViewDist(IncompleteDecodeError),
    ChatMode(VarIntDecodeError),
    UnknownChatMode(u32),
    ChatColours(IncompleteDecodeError),
    SkinFlags(IncompleteDecodeError),
    MainHand(VarIntDecodeError),
    UnknownMainHand(u32),
    TextFiltered(IncompleteDecodeError),
    AllowMotdListing(IncompleteDecodeError),
    ParticleStatus(VarIntDecodeError),
    UnknownParticleStatus(u32)
}
impl Display for ClientInfoDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Locale(err)              => write!(f, "locale {err}"),
        Self::ViewDist(err)            => write!(f, "view dist {err}"),
        Self::ChatMode(err)            => write!(f, "chat mode {err}"),
        Self::UnknownChatMode(v)       => write!(f, "unknown chat mode {v}"),
        Self::ChatColours(err)         => write!(f, "chat colours {err}"),
        Self::SkinFlags(err)           => write!(f, "skin flags {err}"),
        Self::MainHand(err)            => write!(f, "main hand {err}"),
        Self::UnknownMainHand(v)       => write!(f, "unknown main hand {v}"),
        Self::TextFiltered(err)        => write!(f, "text filtered {err}"),
        Self::AllowMotdListing(err)    => write!(f, "allow MOTD listing {err}"),
        Self::ParticleStatus(err)      => write!(f, "particle status {err}"),
        Self::UnknownParticleStatus(v) => write!(f, "unknown particle status {v}")
    } }
}
