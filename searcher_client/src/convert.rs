//! Contains utility functions convert to/from protos and Rust structs.

use std::{
    cmp::min,
    net::{IpAddr, Ipv4Addr},
};
use bincode::serialize;
use jito_protos::packet::{
    Meta as ProtoMeta, Packet as ProtoPacket, PacketBatch as ProtoPacketBatch,
    PacketFlags as ProtoPacketFlags,
};
use solana_perf::packet::{Meta, Packet, PacketBatch, PacketFlags, PACKET_DATA_SIZE};
use solana_sdk::transaction::VersionedTransaction;

/// Converts a Solana packet to a protobuf packet
/// NOTE: the packet.data() function will filter packets marked for discard
pub fn packet_to_proto_packet(p: &Packet) -> Option<ProtoPacket> {
    let meta = p.meta();
    Some(ProtoPacket {
        data: p.data(..)?.to_vec(),
        meta: Some(ProtoMeta {
            size: meta.size as u64,
            addr: meta.addr.to_string(),
            port: meta.port as u32,
            flags: Some(ProtoPacketFlags {
                discard: meta.discard(),
                forwarded: meta.forwarded(),
                repair: meta.repair(),
                from_staked_node: meta.is_from_staked_node(),
                simple_vote_tx: p.meta().is_simple_vote_tx(),
                tracer_packet: false,
            }),
            sender_stake: 0,
        }),
    })
}

// pub fn packet_batches_to_proto_packets(
//     batches: &[PacketBatch],
// ) -> impl Iterator<Item = ProtoPacket> + '_ {
//     batches.iter().flat_map(|b| {
//         b.iter()
//             .filter(|p| !p.meta.discard())
//             .filter_map(packet_to_proto_packet)
//     })
// }

// const UNKNOWN_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
/// converts from a protobuf packet to packet
// pub fn proto_packet_to_packet(p: &ProtoPacket) -> Packet {
//     let mut data = [0; PACKET_DATA_SIZE];
//     let copy_len = min(data.len(), p.data.len());
//     data[..copy_len].copy_from_slice(&p.data[..copy_len]);
//     let mut packet = Packet::new(data, Default::default());
//     if let Some(meta) = &p.meta {
//         let packet_meta = packet.meta_mut();
//         packet_meta.size = meta.size as usize;
//         packet_meta.addr = meta.addr.parse().unwrap_or(UNKNOWN_IP);
//         packet_meta.port = meta.port as u16;
//         if let Some(flags) = &meta.flags {
//             if flags.simple_vote_tx {
//                 packet_meta.flags.insert(PacketFlags::SIMPLE_VOTE_TX);
//             }
//             if flags.forwarded {
//                 packet_meta.flags.insert(PacketFlags::FORWARDED);
//             }
//             if flags.tracer_packet {
//                 packet_meta.flags.insert(PacketFlags::TRACER_PACKET);
//             }
//             if flags.repair {
//                 packet_meta.flags.insert(PacketFlags::REPAIR);
//             }
//         }
//         // packet_meta.sender_stake = meta.sender_stake;
//     }
//     packet
// }

/// Converts a protobuf packet to a VersionedTransaction
pub fn versioned_tx_from_packet(p: &ProtoPacket) -> Option<VersionedTransaction> {
    let mut data = [0; PACKET_DATA_SIZE];
    let copy_len = min(data.len(), p.data.len());
    data[..copy_len].copy_from_slice(&p.data[..copy_len]);
    let mut packet = Packet::new(data, Default::default());
    if let Some(meta) = &p.meta {
        packet.meta_mut().size = meta.size as usize;
    }
    packet.deserialize_slice(..).ok()
}

/// Converts a VersionedTransaction to a protobuf packet
///
pub fn proto_packet_from_versioned_tx(tx: &VersionedTransaction) -> ProtoPacket {
    let data = serialize(tx).expect("serializes");
    let size = data.len() as u64;
    ProtoPacket {
        data,
        meta: Some(ProtoMeta {
            size,
            addr: "".to_string(),
            port: 0,
            flags: None,
            sender_stake: 0,
        }),
    }
}
