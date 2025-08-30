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
            login::S2CLoginPackets
        }
    },
    value::text::Text
};
use serde_json::to_string as to_json_string;


#[derive(Debug)]
pub struct S2CLoginDisconnectPacket {
    reason_json : String
}

impl PacketMeta for S2CLoginDisconnectPacket {
    const STATE  : PacketState = PacketState::Login;
    const BOUND  : PacketBound = PacketBound::C2S;
    const PREFIX : u8          = 0x00; // TODO: Check against current datagen.
}

unsafe impl PacketEncode for S2CLoginDisconnectPacket {

    #[inline(always)]
    fn encode_len(&self) -> usize {
        self.reason_json.encode_len()
    }

    #[inline(always)]
    unsafe fn encode(&self, buf : &mut EncodeBuf) { unsafe {
        self.reason_json.encode(buf);
    } }

}

impl<'l> From<S2CLoginDisconnectPacket> for S2CPackets<'l> {
    #[inline(always)]
    fn from(value : S2CLoginDisconnectPacket) -> Self { Self::Login(value.into()) }
}

impl<'l> From<S2CLoginDisconnectPacket> for S2CLoginPackets<'l> {
    #[inline(always)]
    fn from(value : S2CLoginDisconnectPacket) -> Self { Self::Disconnect(value) }
}


impl<S> From<S> for S2CLoginDisconnectPacket
where
    S : Into<Text>
{
    #[inline]
    fn from(value : S) -> Self {
        Self { reason_json : to_json_string(&value.into()).unwrap() }
    }
}
