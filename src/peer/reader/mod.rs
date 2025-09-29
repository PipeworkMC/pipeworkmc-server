use pipeworkmc_data::redacted::Redacted;
use std::{
    collections::VecDeque,
    net::TcpStream
};
use bevy_ecs::component::Component;
use openssl::symm::Crypter;


mod read;
pub(in crate::peer) use read::*;
mod decode;
pub(in crate::peer) use decode::*;


#[derive(Component)]
pub(in crate::peer) struct PeerStreamReader {
    stream           : TcpStream,
    decrypter        : Option<Redacted<Crypter>>,
    bytes_to_decode  : VecDeque<u8>,
    next_packet_size : Option<usize>
}

impl PeerStreamReader {

    #[inline]
    pub(in crate::peer) fn new(stream : TcpStream) -> Self { Self {
        stream,
        decrypter        : None,
        bytes_to_decode  : VecDeque::new(),
        next_packet_size : None
    } }

    #[inline]
    pub(in crate::peer) fn set_decrypter(&mut self, decrypter : Redacted<Crypter>) {
        self.decrypter = Some(decrypter);
    }

}
