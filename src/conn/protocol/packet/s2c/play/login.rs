use crate::conn::protocol::{
    codec::encode::{
        PacketEncode,
        EncodeBuf
    },
    packet::{
        PacketMeta,
        PacketState,
        PacketBound,
        s2c::{
            S2CPackets,
            play::S2CPlayPackets
        }
    }
};
use crate::data::{
    block_pos::DimBlockPos,
    character::CharacterId,
    ident::Ident,
    game_mode::GameMode,
    varint::VarInt
};
use std::borrow::Cow;


#[derive(Debug)]
pub struct S2CPlayLoginPacket<'l> {
    pub eid                  : CharacterId,
    pub hardcore             : bool,
    pub all_dim_ids          : Cow<'l, [Ident]>,
    pub max_players          : u32,
    pub view_dist            : u32,
    pub sim_dist             : u32,
    pub reduced_debug_info   : bool,
    pub respawn_screen       : bool,
    pub limited_crafting     : bool,
    pub dim_type             : u32,
    pub dim_id               : Ident,
    pub hashed_seed          : u64,
    pub game_mode            : GameMode,
    pub prev_game_mode       : Option<GameMode>,
    pub is_debug_world       : bool,
    pub is_flat_world        : bool,
    pub death_location       : Option<DimBlockPos>,
    pub portal_cooldown      : u32,
    pub sea_level            : i32,
    pub enforces_secure_chat : bool
}

impl PacketMeta for S2CPlayLoginPacket<'_> {
    const STATE  : PacketState = PacketState::Play;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x2B; // TODO: Check against current datagen.
}

unsafe impl PacketEncode for S2CPlayLoginPacket<'_> {

    fn encode_len(&self) -> usize {
        self.eid.as_u32().encode_len()
        + self.hardcore.encode_len()
        + self.all_dim_ids.encode_len()
        + VarInt::<u32>(self.max_players).encode_len()
        + VarInt::<u32>(self.view_dist).encode_len()
        + VarInt::<u32>(self.sim_dist).encode_len()
        + self.reduced_debug_info.encode_len()
        + self.respawn_screen.encode_len()
        + self.limited_crafting.encode_len()
        + VarInt::<u32>(self.dim_type).encode_len()
        + self.dim_id.encode_len()
        + self.hashed_seed.encode_len()
        + (self.game_mode as u8).encode_len()
        + self.prev_game_mode.map_or(-1, |g| g as i8).encode_len()
        + self.is_debug_world.encode_len()
        + self.is_flat_world.encode_len()
        + self.death_location.encode_len()
        + VarInt::<u32>(self.portal_cooldown).encode_len()
        + VarInt::<i32>(self.sea_level).encode_len()
        + self.enforces_secure_chat.encode_len()
    }

    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.eid.as_u32().encode(buf);
        self.hardcore.encode(buf);
        self.all_dim_ids.encode(buf);
        VarInt::<u32>(self.max_players).encode(buf);
        VarInt::<u32>(self.view_dist).encode(buf);
        VarInt::<u32>(self.sim_dist).encode(buf);
        self.reduced_debug_info.encode(buf);
        self.respawn_screen.encode(buf);
        self.limited_crafting.encode(buf);
        VarInt::<u32>(self.dim_type).encode(buf);
        self.dim_id.encode(buf);
        self.hashed_seed.encode(buf);
        (self.game_mode as u8).encode(buf);
        self.prev_game_mode.map_or(-1, |g| g as i8).encode(buf);
        self.is_debug_world.encode(buf);
        self.is_flat_world.encode(buf);
        self.death_location.encode(buf);
        VarInt::<u32>(self.portal_cooldown).encode(buf);
        VarInt::<i32>(self.sea_level).encode(buf);
        self.enforces_secure_chat.encode(buf);
    } }

}

impl<'l> From<S2CPlayLoginPacket<'l>> for S2CPackets<'l> {
    #[inline(always)]
    fn from(value : S2CPlayLoginPacket<'l>) -> Self { Self::Play(value.into()) }
}

impl<'l> From<S2CPlayLoginPacket<'l>> for S2CPlayPackets<'l> {
    #[inline(always)]
    fn from(value : S2CPlayLoginPacket<'l>) -> Self { Self::Login(value) }
}
