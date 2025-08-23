pub mod handshake;
pub mod status;
pub mod login;


#[derive(Debug)]
pub enum C2SPackets {
    Handshake(handshake::C2SHandshakePackets),
    Status(status::C2SStatusPackets),
    Login(login::C2SLoginPackets)
    // TODO: Config
    // TODO: Play
}
