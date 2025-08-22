pub mod status;


#[derive(Debug)]
pub enum S2CPackets {
    Status(status::S2CStatusPackets),
    // TODO: Login
    // TODO: Config
    // TODO: Play
}
