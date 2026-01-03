//! Multi-Chain Archivation V15 — Lightning Network Swarm + Mission Control
//!
//! Layers:
//! - Lightning (Rot0/Rot90): Dynamic fast layer, keysend broadcast
//! - Solana (Rot90): Fast on-chain layer
//! - Ethereum (Rot180): Balanced layer
//! - Bitcoin (Rot180): Immortal OP_RETURN layer
//!
//! Mission Control: Probabilistic pathfinding with learning (T/G + consciousness)

use std::collections::HashMap;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use tracing::info;
use chrono::Utc;
use hex;

use crate::genome::{Genome, hash_genome_dna};
use crate::rotation::Rot180;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockchainLayer {
    Lightning,   // Dynamic, keysend 0-sat
    Solana,      // Fast on-chain
    Ethereum,    // Balanced
    Bitcoin,     // Immortal OP_RETURN
}

impl BlockchainLayer {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Lightning => "Lightning",
            Self::Solana => "Solana",
            Self::Ethereum => "Ethereum",
            Self::Bitcoin => "Bitcoin",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Lightning => "⚡",
            Self::Solana => "🔵",
            Self::Ethereum => "🔷",
            Self::Bitcoin => "🟠",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainArchiveEntry {
    pub layer: BlockchainLayer,
    pub genome_id: i64,
    pub consciousness: u32,
    pub tg_ratio: f64,
    pub dna_hash: String,
    pub tx_hash: Option<String>,
    pub timestamp: i64,
}

#[derive(Debug, Clone)]
struct LayerPair {
    probability: f64,
    last_update: i64,
}

pub struct MultiChainArchiver {
    archives: Vec<ChainArchiveEntry>,
    mock_tx_counter: u64,
    half_life_secs: f64,
    pairs: HashMap<(BlockchainLayer, BlockchainLayer), LayerPair>,
}

impl MultiChainArchiver {
    pub fn new() -> Self {
        info!("⚡ MultiChainArchiver V15 initialized");
        info!("Own pubkey: 02divine...placeholder");
        info!("Swarm nodes: 1");
        Self {
            archives: Vec::new(),
            mock_tx_counter: 0,
            half_life_secs: 3600.0, // 1 час half-life
            pairs: HashMap::new(),
        }
    }

    pub async fn archive<R: crate::rotation::Rotation>(&mut self, genome: &Genome<R>) -> anyhow::Result<ChainArchiveEntry> {
        let dna = genome.to_dna_string();
        let dna_hash = hex::encode(hash_genome_dna(&dna));
        let consciousness = genome.consciousness;
        let tg_ratio = genome.rna_signal();

        let layer = self.select_layer(consciousness, tg_ratio);
        let tx_hash = self.generate_mock_tx(&dna, layer);

        let entry = ChainArchiveEntry {
            layer,
            genome_id: genome.db_id.unwrap_or(0),
            consciousness,
            tg_ratio,
            dna_hash: dna_hash.clone(),
            tx_hash: Some(tx_hash.clone()),
            timestamp: Utc::now().timestamp(),
        };

        self.archives.push(entry.clone());

        info!("{} Archive: genome #{} → {} | consciousness {} | T/G {:.2} | TX: {}",
              layer.emoji(), genome.db_id.unwrap_or(0), layer.name(),
              consciousness, tg_ratio, tx_hash);

        Ok(entry)
    }

    fn select_layer(&self, consciousness: u32, tg_ratio: f64) -> BlockchainLayer {
        if consciousness > 1200 {
            if tg_ratio > 1.5 {
                BlockchainLayer::Lightning
            } else if tg_ratio < 0.5 {
                BlockchainLayer::Bitcoin
            } else {
                BlockchainLayer::Ethereum
            }
        } else {
            BlockchainLayer::Solana
        }
    }

    fn generate_mock_tx(&mut self, data: &str, layer: BlockchainLayer) -> String {
        self.mock_tx_counter += 1;
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hasher.update(layer.name().as_bytes());
        hasher.update(Utc::now().timestamp_nanos_opt().unwrap_or(0).to_le_bytes());
        hasher.update(self.mock_tx_counter.to_le_bytes());
        let hash = hasher.finalize();

        match layer {
            BlockchainLayer::Lightning => format!("ln_{}", hex::encode(&hash[..8])),
            BlockchainLayer::Solana => format!("sol_{}", hex::encode(&hash[..32])),
            BlockchainLayer::Ethereum => format!("0x{}", hex::encode(hash)),
            BlockchainLayer::Bitcoin => format!("0x{}", hex::encode(hash)),
        }
    }

    // ФИКС E0503: копируем half_life до mutable borrow
    pub fn get_probability(&mut self, from: BlockchainLayer, to: BlockchainLayer) -> f64 {
        let half_life = self.half_life_secs;
        let pair = self.pairs.entry((from, to)).or_insert(LayerPair {
            probability: 0.95,
            last_update: Utc::now().timestamp(),
        });

        // Time decay
        let now = Utc::now().timestamp();
        let age = (now - pair.last_update) as f64;
        pair.probability *= (-age / half_life).exp();
        pair.last_update = now;

        pair.probability
    }

    pub fn recent_archives(&self, limit: usize) -> Vec<&ChainArchiveEntry> {
        self.archives.iter().rev().take(limit).collect()
    }
}

impl Default for MultiChainArchiver {
    fn default() -> Self {
        Self::new()
    }
}