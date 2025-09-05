use crate::conn::protocol::{
    codec::decode::{
        PacketDecode,
        PrefixedPacketDecode,
        DecodeBuf,
        IncompleteDecodeError
    }
};
use core::{
    fmt::{ self, Display, Formatter },
    hint::unreachable_unchecked
};


#[derive(Debug)]
pub enum C2SPlayPackets {
    // TODO: ConfirmTeleport
    // TODO: QueryBlockEntityTag
    // TODO: BundleItemSelected
    // TODO: ChangeDifficulty
    // TODO: ChangeGameMode
    // TODO: AcknowledgeMessage
    // TODO: ChatCommand
    // TODO: SignedChatCommand
    // TODO: ChatMessage
    // TODO: PlayerSession
    // TODO: ChunkBatchReceived
    // TODO: ClientStatus
    // TODO: ClientTickEnd
    // TODO: ClientInfo
    // TODO: CommandSuggestionsRequest
    // TODO: AcknowledgeConfig
    // TODO: ClickContainerButton
    // TODO: ClickContainer
    // TODO: CloseContinaer
    // TODO: ChangeContainerSlotState
    // TODO: CookieResponse
    // TODO: CustomPayload
    // TODO: DebugSampleSubscription
    // TODO: EditBook
    // TODO: QueryEntityTag
    // TODO: Interact
    // TODO: JigsawGenerate
    // TODO: KeepAlive
    // TODO: LockDifficulty
    // TODO: SetPlayerPos
    // TODO: SetPlayerPosAndRot
    // TODO: SetPlayerRot
    // TODO: SetPlayerMovementFlags
    // TODO: MoveVehicle
    // TODO: PaddleBoat
    // TODO: PickBlock
    // TODO: PickEntity
    // TODO: PingRequest
    // TODO: PlaceRecipe
    // TODO: PlayerAbilities
    // TODO: PlayerAction
    // TODO: PlayerCommand
    // TODO: PlayerInput
    // TODO: PlayerLoaded
    // TODO: Pong
    // TODO: ChangeRecipeBookSettings
    // TODO: SetSeenRecipe
    // TODO: RenameItem
    // TODO: ResourcePackResponse
    // TODO: SeenAdvancements
    // TODO: SelectTrade
    // TODO: SetBeaconEffect
    // TODO: SetHeldItem
    // TODO: ProgramCommandBlock
    // TODO: ProgramCommandBlockMinecart
    // TODO: SetCreativeModeSlot
    // TODO: ProgramJigsawBlock
    // TODO: ProgramStructureBlock
    // TODO: SetTestBlock
    // TODO: UpdateSign
    // TODO: SwingArm
    // TODO: TeleportToEntity
    // TODO: TestInstanceBlockAction
    // TODO: UseItemOn
    // TODO: UseItem
    // TODO: CustomClickAction
}

impl PrefixedPacketDecode for C2SPlayPackets {
    type Error = C2SPlayDecodeError;

    fn decode_prefixed(buf : &mut DecodeBuf<'_>)
        -> Result<Self, Self::Error>
    {
        Ok(match (buf.read().map_err(C2SPlayDecodeError::Incomplete)?) {

            v => { return Err(C2SPlayDecodeError::UnknownPrefix(v)); }
        })
    }
}


#[derive(Debug)]
pub enum C2SPlayDecodeError {
    Incomplete(IncompleteDecodeError),
    UnknownPrefix(u8)
}
impl From<!> for C2SPlayDecodeError {
    #[inline(always)]
    fn from(_ : !) -> Self { unsafe { unreachable_unchecked() } }
}
impl Display for C2SPlayDecodeError {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result { match (self) {
        Self::Incomplete(err)  => err.fmt(f),
        Self::UnknownPrefix(b) => write!(f, "unknown prefix `0x{b:0>2x}`"),
    } }
}
