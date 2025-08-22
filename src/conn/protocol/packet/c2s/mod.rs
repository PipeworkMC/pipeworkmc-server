pub mod handshake;


#[derive(Debug)]
pub enum C2SPackets {
    Handshake(handshake::C2SHandshakePackets),
    // TODO: Status
    // TODO: Login
    // TODO: Config
    // TODO: Play
}
