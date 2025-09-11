use crate::conn::protocol::{
    codec::{
        encode::{
            PrefixedPacketEncode,
            EncodeBuf
        },
        meta::PacketMeta
    },
    packet::s2c::S2CPackets
};


pub mod disconnect;
pub mod keep_alive;
pub mod login;


#[derive(Debug)]
pub enum S2CPlayPackets<'l> {
    // TODO: BundleDelim
    // TODO: SpawnEntity
    // TODO: EntityAnim
    // TODO: AwardStat
    // TODO: AcknowledgeBlockChange
    // TODO: SetBlockDestroyStage
    // TODO: BlockEntityData
    // TODO: BlockAction
    // TODO: BlockUpdate
    // TODO: BossBar
    // TODO: ChangeDifficulty
    // TODO: ChunkBatchFinished
    // TODO: ChunkBatchStart
    // TODO: ChunkBiomes
    // TODO: ClearTitles
    // TODO: CommandSuggestionsResponse
    // TODO: Commands
    // TODO: CloseContainer
    // TODO: SetContainerContent
    // TODO: SetContainerProperty
    // TODO: SetContainerSlot
    // TODO: CookieRequest
    // TODO: SetCooldown
    // TODO: ChatSuggestions
    // TODO: CustomPayload
    // TODO: DamageEvent
    // TODO: DebugSample
    // TODO: DeleteMessage
    Disconnect (disconnect ::S2CPlayDisconnectPacket),
    // TODO: DisguisedChatMessage
    // TODO: EntityEvent
    // TODO: TeleportEntity
    // TODO: Explosion
    // TODO: UnloadChunk
    // TODO: GameEvent
    // TODO: OpenHorseScreen
    // TODO: HurtAnim
    // TODO: InitWorldBorder
    KeepAlive  (keep_alive ::S2CPlayKeepAlivePacket),
    // TODO: ChunkDataAndLightUpdate
    // TODO: WorldEvent
    // TODO: Particle
    // TODO: UpdateLight
    Login      (login      ::S2CPlayLoginPacket<'l>)
    // TODO: MapData
    // TODO: MerchantOffers
    // TODO: UpdateEntityPos
    // TODO: UpdateEntityPosAndRot
    // TODO: MoveMinecartAlongTrack
    // TODO: UpdateEntityRot
    // TODO: MoveVehicle
    // TODO: OpenBook
    // TODO: OpenScreen
    // TODO: OpenSignEditor
    // TODO: Ping
    // TODO: PingResponse
    // TODO: PlaceGhostRecipe
    // TODO: PlayerAbilities
    // TODO: PlayerChatMessage
    // TODO: EndCombat
    // TODO: EnterCombat
    // TODO: CombatDeath
    // TODO: PlayerInfoRemove
    // TODO: PlayerInfoUpdate
    // TODO: LookAt
    // TODO: SynchronisePlayerPos
    // TODO: PlayerRot
    // TODO: RecipeBookAdd
    // TODO: RecipeBookRemove
    // TODO: RecipeBookSettings
    // TODO: RemoveEntities
    // TODO: RemoveEntityEffect
    // TODO: ResetScore
    // TODO: RemoveResourcePack
    // TODO: AddResourcePack
    // TODO: Respawn
    // TODO: SetHeadRot
    // TODO: UpdateSectionBlocks
    // TODO: SelectAdvancementsTab
    // TODO: ServerData
    // TODO: SetActionBarText
    // TODO: SetBorderCentre
    // TODO: SetBorderLerpSize
    // TODO: SetBorderSize
    // TODO: SetBorderWarningDelay
    // TODO: SetBorderWarningDist
    // TODO: SetCamera
    // TODO: SetCentreChunk
    // TODO: SetRenderDist
    // TODO: SetCursorItem
    // TODO: SetDefaultSpawnPos
    // TODO: DisplayObjective
    // TODO: SetEntityMeta
    // TODO: LinkEntities
    // TODO: SetEntityVel
    // TODO: SetEquipment
    // TODO: SetExperience
    // TODO: SetHealth
    // TODO: SetHeldItem
    // TODO: UpdateObjectives
    // TODO: SetPassengers
    // TODO: SetPlayerInvSlot
    // TODO: UpdateTeams
    // TODO: UpdateScore
    // TODO: SetSimulationDist
    // TODO: SetSubtitleText
    // TODO: UpdateTime
    // TODO: SetTitleText
    // TODO: SetTitleAnimTimes
    // TODO: EntitySoundEffect
    // TODO: SoundEffect
    // TODO: StartConfig
    // TODO: StopSound
    // TODO: StoreCookie
    // TODO: SystemChatMessage
    // TODO: SetTabListHeader
    // TODO: TagQueryResponse
    // TODO: PickupItem
    // TODO: SynchroniseVehiclePos
    // TODO: TestInstanceBlockStatus
    // TODO: SetTickingState
    // TODO: StepTick
    // TODO: Transfer
    // TODO: UpdateAdvancements
    // TODO: UpdateAttributes
    // TODO: EntityEffect
    // TODO: UpdateRecipes
    // TODO: UpdateTags
    // TODO: ProjectilePower
    // TODO: CustomReportDetails
    // TODO: ServerLinks
    // TODO: Waypoint
    // TODO: ClearDialog
    // TODO: ShowDialog
}

impl S2CPlayPackets<'_> {

    pub fn prefix(&self) -> u8 { match (self) {
        Self::Disconnect (_) => disconnect ::S2CPlayDisconnectPacket ::PREFIX,
        Self::KeepAlive  (_) => keep_alive ::S2CPlayKeepAlivePacket  ::PREFIX,
        Self::Login      (_) => login      ::S2CPlayLoginPacket      ::PREFIX
    } }

}

unsafe impl PrefixedPacketEncode for S2CPlayPackets<'_> {

    fn encode_prefixed_len(&self) -> usize { match (self) {
        Self::Disconnect (packet) => packet.encode_prefixed_len(),
        Self::KeepAlive  (packet) => packet.encode_prefixed_len(),
        Self::Login      (packet) => packet.encode_prefixed_len()
    } }

    unsafe fn encode_prefixed(&self, buf : &mut EncodeBuf) { unsafe { match (self) {
        Self::Disconnect (packet) => packet.encode_prefixed(buf),
        Self::KeepAlive  (packet) => packet.encode_prefixed(buf),
        Self::Login      (packet) => packet.encode_prefixed(buf)
    } } }

}

impl<'l> From<S2CPlayPackets<'l>> for S2CPackets<'l> {
    #[inline(always)]
    fn from(value : S2CPlayPackets<'l>) -> Self { Self::Play(value) }
}
