pub mod c2s;
pub mod s2c;


pub trait PacketMeta {
    const STATE  : PacketState;
    const BOUND  : PacketBound;
    const PREFIX : u8;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub enum PacketState {
    #[default]
    Handshake,
    Status,
    Login,
    Config,
    Play
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PacketBound {
    S2C,
    C2S
}
