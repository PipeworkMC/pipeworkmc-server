use crate::peer::{
    Peer,
    reader::PeerStreamReader,
    writer::PacketSender,
    state::PeerState,
    message::{
        PacketReceived,
        SendPacket
    }
};
use crate::util::VecDequeExt;
use pipeworkmc_codec::{
    decode::{
        PrefixedPacketDecode,
        DecodeIter
    },
    meta::PacketState
};
use pipeworkmc_data::varint::{
    VarIntType,
    VarIntDecodeError
};
use pipeworkmc_packet::c2s::{
    C2SPackets,
    handshake::{
        C2SHandshakePackets,
        intention::{
            C2SHandshakeIntentionPacket,
            Intention
        }
    },
    status::C2SStatusPackets,
    login::{
        C2SLoginPackets,
        finish_acknowledged::C2SLoginFinishAcknowledgedPacket
    },
    config::{
        C2SConfigPackets,
        finish_acknowledged::C2SConfigFinishAcknowledgedPacket
    },
    play::C2SPlayPackets
};
use bevy_ecs::{
    entity::Entity,
    query::With,
    system::Query
};
use bevy_parmessagewriter::ParallelMessageWriter;


/// Decodes bytes that have been read from the stream into packets, sending them out as messages.
pub(in crate::peer) fn decode_peer_packets(
    mut q_peers     : Query<(Entity, &mut PeerStreamReader, &mut PeerState), (With<Peer>,)>,
        mw_received : ParallelMessageWriter<PacketReceived>,
        mw_send     : ParallelMessageWriter<SendPacket>
) {
    q_peers.par_iter_mut().for_each(|(entity, mut reader, mut state)| {
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

        // If enough bytes are present, decode a packet.
        if let Some(packet_size) = packet_size {
            if (packet_size > 2097151) {
                // TODO: Cap packet length.
                // TODO: Cap uncompressed packet length (8388608).
            }

            if (reader.bytes_to_decode.len() >= packet_size) {
                reader.next_packet_size = None;
                let mut iter = DecodeIter::from(reader.bytes_to_decode.iter().cloned());
                // TODO: Decompress
                match (state.incoming()) {

                    PacketState::Handshake => { match (C2SHandshakePackets::decode_prefixed(&mut iter)) {
                        Ok(packet) => {
                            let C2SHandshakePackets::Intention(C2SHandshakeIntentionPacket { intent, .. }) = &packet;
                            match (intent) {
                                Intention::Status       => { state.both_to_status(); },
                                Intention::Login { .. } => { state.both_to_login(); }
                            }
                            mw_received.write(PacketReceived::new(entity, C2SPackets::Handshake(packet)));
                        },
                        Err(err)   => { mw_send.write(SendPacket::new(entity).kick_packet_error(format!("handshake {err}"))); }
                    } },

                    PacketState::Status => { match (C2SStatusPackets::decode_prefixed(&mut iter)) {
                        Ok(packet) => { mw_received.write(PacketReceived::new(entity, C2SPackets::Status(packet))); },
                        Err(err)   => { mw_send.write(SendPacket::new(entity).kick_packet_error(format!("status {err}"))); }
                    } },

                    PacketState::Login => { match (C2SLoginPackets::decode_prefixed(&mut iter)) {
                        Ok(packet) => {
                            if let C2SLoginPackets::FinishAcknowledged(C2SLoginFinishAcknowledgedPacket { }) = packet {
                                state.incoming_to_config();
                            }
                            mw_received.write(PacketReceived::new(entity, C2SPackets::Login(packet)));
                        },
                        Err(err)   => { mw_send.write(SendPacket::new(entity).kick_packet_error(format!("login {err}"))); }
                    } },

                    PacketState::Config => { match (C2SConfigPackets::decode_prefixed(&mut iter)) {
                        Ok(packet) => {
                            if let C2SConfigPackets::FinishAcknowledged(C2SConfigFinishAcknowledgedPacket { }) = packet {
                                state.incoming_to_play();
                            }
                            mw_received.write(PacketReceived::new(entity, C2SPackets::Config(packet)));
                        },
                        Err(err)   => { mw_send.write(SendPacket::new(entity).kick_packet_error(format!("config {err}"))); }
                    } },

                    PacketState::Play => { match (C2SPlayPackets::decode_prefixed(&mut iter)) {
                        Ok(packet) => { mw_received.write(PacketReceived::new(entity, C2SPackets::Play(packet))); }, // TODO: Handle configuration acknowledged.
                        Err(err)   => { mw_send.write(SendPacket::new(entity).kick_packet_error(format!("play {err}"))); }
                    } }

                };
                reader.bytes_to_decode.pop_many_front(packet_size);
            }
        }

    });
}
