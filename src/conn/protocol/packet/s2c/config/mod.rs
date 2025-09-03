use crate::{conn::protocol::{
    codec::encode::{
        EncodeBuf, PrefixedPacketEncode
    },
    packet::{
        s2c::S2CPackets, PacketMeta
    }
}};


pub mod custom_payload;
pub mod disconnect;
pub mod finish;


#[derive(Debug)]
pub enum S2CConfigPackets<'l> {
    // TODO: CookieRequest
    CustomPayload (custom_payload ::S2CConfigCustomPayloadPacket<'l>),
    Disconnect    (disconnect ::S2CConfigDisconnectPacket),
    Finish        (finish     ::S2CConfigFinishPacket),
    // TODO: Keepalive
    // TODO: Ping
    // TODO: ResetChat
    // TODO: RegistryData
    // TODO: RemoveResourcePack
    // TODO: AddResourcePack
    // TODO: StoreCookie
    // TODO: Transfer
    // TODO: FeatureFlags
    // TODO: UpdateTags
    // TODO: KnownPacks
    // TODO: CustomReportDetails
    // TODO: ServerLinks
    // TODO: ClearDialog
    // TODO: ShowDialog
}

impl S2CConfigPackets<'_> {

    pub fn prefix(&self) -> u8 { match (self) {
        Self::CustomPayload (_) => custom_payload ::S2CConfigCustomPayloadPacket ::PREFIX,
        Self::Disconnect    (_) => disconnect     ::S2CConfigDisconnectPacket    ::PREFIX,
        Self::Finish        (_) => finish         ::S2CConfigFinishPacket        ::PREFIX
    } }

}

unsafe impl PrefixedPacketEncode for S2CConfigPackets<'_> {

    fn encode_prefixed_len(&self) -> usize { match (self) {
        Self::CustomPayload (packet) => packet.encode_prefixed_len(),
        Self::Disconnect    (packet) => packet.encode_prefixed_len(),
        Self::Finish        (packet) => packet.encode_prefixed_len()
    } }

    unsafe fn encode_prefixed(&self, buf : &mut EncodeBuf) { unsafe { match (self) {
        Self::CustomPayload (packet) => packet.encode_prefixed(buf),
        Self::Disconnect    (packet) => packet.encode_prefixed(buf),
        Self::Finish        (packet) => packet.encode_prefixed(buf)
    } } }

}

impl<'l> From<S2CConfigPackets<'l>> for S2CPackets<'l> {
    #[inline(always)]
    fn from(value : S2CConfigPackets<'l>) -> Self { Self::Config(value) }
}
