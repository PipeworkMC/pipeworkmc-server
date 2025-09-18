use crate::peer::{
    PeerAddress,
    reader::PeerStreamReader,
    writer::PacketSender,
    state::PeerState,
    event::{
        PacketReceived,
        SendPacket
    }
};
use crate::ecs::ParallelEventWriter;
use crate::util::VecDequeExt;
use pipeworkmc_codec::{
    decode::{
        PrefixedPacketDecode,
        DecodeBuf
    },
    meta::PacketState
};
use pipeworkmc_data::varint::{
    VarIntType,
    VarIntDecodeError
};
use pipeworkmc_packet::c2s::{
    C2SPackets,
    handshake::C2SHandshakePackets,
    status::C2SStatusPackets,
    login::C2SLoginPackets,
    config::C2SConfigPackets,
    play::C2SPlayPackets
};
use bevy_ecs::{
    entity::Entity,
    query::With,
    system::Query
};


pub(in crate::peer) fn decode_peer_packets(
    mut q_peers     : Query<(Entity, &mut PeerStreamReader, &PeerState), (With<PeerAddress>,)>,
        ew_received : ParallelEventWriter<PacketReceived>,
        ew_send     : ParallelEventWriter<SendPacket>
) {
    q_peers.par_iter_mut().for_each(|(entity, mut reader, state)| {
        if (state.disconnecting()) { return; }

        // Get or try to decode next packet size.
        let packet_size = match (reader.next_packet_size) {
            Some(v) => Some(v),
            None    => {
                let v = match (<u32 as VarIntType>::decode(reader.bytes_to_decode.iter().cloned())) {
                    Ok((next_size, consumed,)) => {
                        reader.bytes_to_decode.pop_many_front(consumed);
                        Some(next_size as usize)
                    },
                    Err(VarIntDecodeError::Incomplete(_)) => None,
                    Err(VarIntDecodeError::TooLong)       => panic!("{:?}", VarIntDecodeError::TooLong), // TODO: Error handler.
                };
                reader.next_packet_size = v;
                v
            }
        };

        // TODO: Cap packet length.

        // If enough bytes are present, decode a packet.
        if let Some(packet_size) = packet_size
            && (reader.bytes_to_decode.len() >= packet_size)
        {
            reader.next_packet_size = None;
            let     buf = unsafe { reader.bytes_to_decode.pop_many_front_into_unchecked(packet_size) };
            let mut buf = DecodeBuf::from(&*buf);
            // TODO: Decompress
            match (state.incoming()) {
                PacketState::Handshake => { match (C2SHandshakePackets::decode_prefixed(&mut buf)) {
                    Ok(packet) => { ew_received.write(PacketReceived::new(entity, C2SPackets::Handshake(packet))); },
                    Err(err)   => { ew_send.write(SendPacket::new(entity).kick_packet_error(format!("handshake {err}"))); }
                } },
                PacketState::Status => { match (C2SStatusPackets::decode_prefixed(&mut buf)) {
                    Ok(packet) => { ew_received.write(PacketReceived::new(entity, C2SPackets::Status(packet))); },
                    Err(err)   => { ew_send.write(SendPacket::new(entity).kick_packet_error(format!("status {err}"))); }
                } },
                PacketState::Login => { match (C2SLoginPackets::decode_prefixed(&mut buf)) {
                    Ok(packet) => { ew_received.write(PacketReceived::new(entity, C2SPackets::Login(packet))); },
                    Err(err)   => { ew_send.write(SendPacket::new(entity).kick_packet_error(format!("login {err}"))); }
                } },
                PacketState::Config => { match (C2SConfigPackets::decode_prefixed(&mut buf)) {
                    Ok(packet) => { ew_received.write(PacketReceived::new(entity, C2SPackets::Config(packet))); },
                    Err(err)   => { ew_send.write(SendPacket::new(entity).kick_packet_error(format!("config {err}"))); }
                } },
                PacketState::Play => { match (C2SPlayPackets::decode_prefixed(&mut buf)) {
                    Ok(packet) => { ew_received.write(PacketReceived::new(entity, C2SPackets::Play(packet))); },
                    Err(err)   => { ew_send.write(SendPacket::new(entity).kick_packet_error(format!("play {err}"))); }
                } }
            };
        }

    });
}
