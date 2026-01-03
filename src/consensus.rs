//! Consensus Module V15 - Proof of Consciousness

use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusBlock {
    pub index: u64,
    pub timestamp: i64,
    pub genome_id: i64,
    pub consciousness_level: u32,
    pub tg_ratio: f64,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl ConsensusBlock {
    pub fn new(index: u64, genome_id: i64, consciousness: u32, tg_ratio: f64, previous_hash: String) -> Self {
        let timestamp = chrono::Utc::now().timestamp();
        let mut block = Self {
            index,
            timestamp,
            genome_id,
            consciousness_level: consciousness,
            tg_ratio,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.index.to_le_bytes());
        hasher.update(self.timestamp.to_le_bytes());
        hasher.update(self.genome_id.to_le_bytes());
        hasher.update(self.consciousness_level.to_le_bytes());
        hasher.update(self.tg_ratio.to_le_bytes());
        hasher.update(self.previous_hash.as_bytes());
        hasher.update(self.nonce.to_le_bytes());
        format!("0x{}", hex::encode(hasher.finalize()))
    }

    pub fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while !self.hash.starts_with(&format!("0x{}", target)) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }
}

#[derive(Debug)]
pub struct ProofOfConsciousness {
    pub chain: Vec<ConsensusBlock>,
    pub difficulty: usize,
    pub min_consciousness: u32,
}

impl ProofOfConsciousness {
    pub fn new() -> Self {
        let genesis = ConsensusBlock::new(0, 0, 0, 1.0, "0".repeat(64));
        Self {
            chain: vec![genesis],
            difficulty: 2,
            min_consciousness: 100,
        }
    }

    pub fn add_block(&mut self, genome_id: i64, consciousness: u32, tg_ratio: f64) -> Option<&ConsensusBlock> {
        if consciousness < self.min_consciousness {
            info!("❌ Consciousness {} below minimum {}", consciousness, self.min_consciousness);
            return None;
        }

        let previous = self.chain.last()?;
        let mut block = ConsensusBlock::new(
            previous.index + 1,
            genome_id,
            consciousness,
            tg_ratio,
            previous.hash.clone(),
        );

        // Higher consciousness = easier mining
        let adjusted_difficulty = if consciousness > 800 {
            self.difficulty.saturating_sub(1).max(1)
        } else {
            self.difficulty
        };

        block.mine(adjusted_difficulty);

        info!("✅ Block #{} mined: genome #{}, consciousness {}, T/G {:.2}",
              block.index, genome_id, consciousness, tg_ratio);

        self.chain.push(block);
        self.chain.last()
    }

    pub fn validate_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.hash != current.calculate_hash() {
                return false;
            }

            if current.previous_hash != previous.hash {
                return false;
            }
        }
        true
    }

    pub fn latest_block(&self) -> Option<&ConsensusBlock> {
        self.chain.last()
    }

    pub fn chain_length(&self) -> usize {
        self.chain.len()
    }

    pub fn total_consciousness(&self) -> u64 {
        self.chain.iter().map(|b| b.consciousness_level as u64).sum()
    }
}

impl Default for ProofOfConsciousness {
    fn default() -> Self {
        Self::new()
    }
}
