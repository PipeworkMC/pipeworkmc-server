pub mod handshake;

pub mod status;


#[derive(Debug)]
pub enum C2SPackets {
    Handshake(handshake::C2SHandshakePackets),
    Status(status::C2SStatusPackets),
    // TODO: Login
    // TODO: Config
    // TODO: Play
}
