use crate::{get_filter, ApiConfig, ApiEvent};
use communication::{network::NetworkConfig, protocol::ProtocolConfig};
use consensus::{BlockGraphExport, ConsensusConfig, ExportCompiledBlock, ExportDiscardedBlocks};
use crypto::{
    hash::Hash,
    signature::{PrivateKey, PublicKey, SignatureEngine},
};
use models::block::{Block, BlockHeader};
use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    vec,
};
use time::UTime;
use tokio::sync::mpsc::{self, Receiver};
use warp::{filters::BoxedFilter, reply::Reply};

pub fn get_test_hash() -> Hash {
    Hash::hash("test".as_bytes())
}

pub fn get_another_test_hash() -> Hash {
    Hash::hash("another test".as_bytes())
}

pub fn get_consensus_config() -> ConsensusConfig {
    ConsensusConfig {
        genesis_timestamp: 0.into(),
        thread_count: 2,
        t0: 2000.into(),
        selection_rng_seed: 0,
        genesis_key: PrivateKey::from_bs58_check(
            "SGoTK5TJ9ZcCgQVmdfma88UdhS6GK94aFEYAsU3F1inFayQ6S",
        )
        .unwrap(),
        nodes: Vec::new(),
        current_node_index: 0,
        max_discarded_blocks: 0,
        future_block_processing_max_periods: 0,
        max_future_processing_blocks: 0,
        max_dependency_blocks: 0,
        delta_f0: 0,
        disable_block_creation: true,
    }
}

pub fn get_protocol_config() -> ProtocolConfig {
    ProtocolConfig {
        message_timeout: 10_000u64.into(),
        ask_peer_list_interval: 10_000u64.into(),
    }
}

pub fn get_network_config() -> NetworkConfig {
    NetworkConfig {
        bind: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
        routable_ip: Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
        protocol_port: 0,
        connect_timeout: UTime::from(180_000),
        wakeup_interval: UTime::from(10_000),
        peers_file: std::path::PathBuf::new(),
        target_out_connections: 10,
        max_in_connections: 5,
        max_in_connections_per_ip: 2,
        max_out_connnection_attempts: 15,
        max_idle_peers: 3,
        max_banned_peers: 3,
        max_advertise_length: 5,
        peers_file_dump_interval: UTime::from(10_000),
    }
}

pub fn get_api_config() -> ApiConfig {
    ApiConfig {
        max_return_invalid_blocks: 5,
        selection_return_periods: 2,
        bind: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3030),
    }
}

pub fn get_header(
    period_number: u64,
    thread_number: u8,
    creator: Option<PublicKey>,
) -> BlockHeader {
    let secp = SignatureEngine::new();
    let private_key = SignatureEngine::generate_random_private_key();
    let public_key = secp.derive_public_key(&private_key);
    BlockHeader {
        creator: if creator.is_none() {
            public_key
        } else {
            creator.unwrap()
        },
        thread_number,
        period_number,
        roll_number: 0,
        parents: Vec::new(),
        endorsements: Vec::new(),
        out_ledger_hash: get_test_hash(),
        operation_merkle_root: get_another_test_hash(),
    }
}

pub fn mock_filter() -> (BoxedFilter<(impl Reply,)>, Receiver<ApiEvent>) {
    let (evt_tx, evt_rx) = mpsc::channel(1);
    (
        get_filter(
            get_api_config(),
            get_consensus_config(),
            get_protocol_config(),
            get_network_config(),
            evt_tx,
        ),
        evt_rx,
    )
}

pub fn get_test_block_graph() -> BlockGraphExport {
    BlockGraphExport {
        genesis_blocks: vec![get_test_hash(), get_another_test_hash()],
        active_blocks: HashMap::new(),
        discarded_blocks: ExportDiscardedBlocks {
            map: HashMap::new(),
        },
        best_parents: Vec::new(),
        latest_final_blocks_periods: Vec::new(),
        gi_head: HashMap::new(),
        max_cliques: Vec::new(),
    }
}

pub fn get_dummy_staker() -> PublicKey {
    let signature_engine = SignatureEngine::new();
    let private_key = SignatureEngine::generate_random_private_key();
    signature_engine.derive_public_key(&private_key)
}

pub fn get_test_block() -> Block {
    Block {
            header: BlockHeader {
                creator: crypto::signature::PublicKey::from_bs58_check("4vYrPNzUM8PKg2rYPW3ZnXPzy67j9fn5WsGCbnwAnk2Lf7jNHb").unwrap(),
                endorsements: vec![],
                operation_merkle_root: get_test_hash(),
                out_ledger_hash: get_test_hash(),
                parents: vec![],
                period_number: 1,
                thread_number: 0,
                roll_number: 0,
            },
            operations: vec![],
            signature: crypto::signature::Signature::from_bs58_check(
                "5f4E3opXPWc3A1gvRVV7DJufvabDfaLkT1GMterpJXqRZ5B7bxPe5LoNzGDQp9LkphQuChBN1R5yEvVJqanbjx7mgLEae"
            ).unwrap()
        }
}

pub fn get_test_compiled_exported_block(
    period: u64,
    thread: u8,
    creator: Option<PublicKey>,
) -> ExportCompiledBlock {
    ExportCompiledBlock {
        block: get_header(period, thread, creator),
        children: Vec::new(),
    }
}