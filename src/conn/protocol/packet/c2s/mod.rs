pub mod handshake;
pub mod status;
pub mod login;
pub mod config;


#[derive(Debug)]
pub enum C2SPackets {
    Handshake(handshake::C2SHandshakePackets),
    Status(status::C2SStatusPackets),
    Login(login::C2SLoginPackets),
    Config(config::C2SConfigPackets)
    // TODO: Play
}
