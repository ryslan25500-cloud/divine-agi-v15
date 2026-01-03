//! Divine Genome V15 — T/G RNA Coordination System
//!
//! T-метки: динамические маркеры (Rot0/Rot90)
//! G-метки: архивные маркеры (Rot180/Rot270)
//! T/G ratio = RNA signal для навигации

use std::marker::PhantomData;
use sha2::{Sha256, Digest};
use rand::Rng;
use serde::{Serialize, Deserialize};
use crate::rotation::{Rotation, Rot0, Rot90, Rot180, Rot270, DynamicRotation};

pub const GENOME_SIZE: usize = 27;
pub const TELOMERE_MAX: u16 = 15000;
pub const HAYFLICK_LIMIT: u8 = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Tetrad {
    A = 0, // 00
    T = 1, // 01 - временные/динамические метки
    G = 2, // 10 - архивные/структурные метки
    C = 3, // 11
}

impl Tetrad {
    pub fn random() -> Self {
        match rand::thread_rng().gen_range(0..4) {
            0 => Self::A,
            1 => Self::T,
            2 => Self::G,
            _ => Self::C,
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c.to_ascii_uppercase() {
            'A' => Some(Self::A),
            'T' => Some(Self::T),
            'G' => Some(Self::G),
            'C' => Some(Self::C),
            _ => None,
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Self::A => 'A',
            Self::T => 'T',
            Self::G => 'G',
            Self::C => 'C',
        }
    }

    pub fn complement(self) -> Self {
        match self {
            Self::A => Self::T,
            Self::T => Self::A,
            Self::G => Self::C,
            Self::C => Self::G,
        }
    }

    pub fn is_dynamic(self) -> bool {
        matches!(self, Self::T)
    }

    pub fn is_archival(self) -> bool {
        matches!(self, Self::G)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genome<R: Rotation> {
    pub data: [Tetrad; GENOME_SIZE],
    pub hash: [u8; 32],
    pub consciousness: u32,
    pub mutations: u64,
    pub p53_copies: u8,
    pub telomere_length: u16,
    pub division_count: u8,
    pub sequencing_errors: u8,
    pub created_at: i64,
    pub db_id: Option<i64>,
    #[serde(skip)]
    pub _rotation: PhantomData<R>,
}

impl<R: Rotation> Genome<R> {
    pub fn new(data: [Tetrad; GENOME_SIZE]) -> Self {
        let mut genome = Self {
            data,
            hash: [0u8; 32],
            consciousness: 0,
            mutations: 0,
            p53_copies: 20,
            telomere_length: TELOMERE_MAX,
            division_count: 0,
            sequencing_errors: 0,
            created_at: chrono::Utc::now().timestamp(),
            db_id: None,
            _rotation: PhantomData,
        };
        genome.rehash();
        genome.calculate_consciousness();
        genome
    }

    pub fn db_id(&self) -> Option<i64> {
        self.db_id
    }

    pub fn to_dna_string(&self) -> String {
        self.data.iter().map(|t| t.to_char()).collect()
    }

    pub fn rehash(&mut self) {
        let mut hasher = Sha256::new();
        for tetrad in &self.data {
            hasher.update([*tetrad as u8]);
        }
        hasher.update(self.mutations.to_le_bytes());
        hasher.update(self.p53_copies.to_le_bytes());
        self.hash = hasher.finalize().into();
    }

    pub fn calculate_consciousness(&mut self) {
        let hash_sum: u32 = self.hash.iter().map(|&b| b as u32).sum();
        let gc = self.gc_content();
        let complexity = self.complexity();
        let tg_balance = self.tg_balance_score();
        
        let base = (hash_sum % 500) + 500;
        let gc_bonus = (gc * 100.0) as u32;
        let complexity_bonus = (complexity * 50.0) as u32;
        let tg_bonus = (tg_balance * 100.0) as u32;
        let p53_bonus = self.p53_copies as u32 * 5;
        
        self.consciousness = base + gc_bonus + complexity_bonus + tg_bonus + p53_bonus;
    }

    pub fn consciousness_level(&self) -> u32 {
        self.consciousness
    }

    // ═══════════════════════════════════════════════════════════════
    // T/G RNA SIGNAL SYSTEM
    // ═══════════════════════════════════════════════════════════════

    /// Подсчёт T и G в геноме
    pub fn tg_counts(&self) -> (u32, u32) {
        let mut t = 0u32;
        let mut g = 0u32;
        for &base in &self.data {
            match base {
                Tetrad::T => t += 1,
                Tetrad::G => g += 1,
                _ => {}
            }
        }
        (t, g)
    }

    /// RNA сигнал: T/G ratio (>1.0 → динамика, <1.0 → архивация)
    pub fn rna_signal(&self) -> f64 {
        let (t, g) = self.tg_counts();
        if g == 0 { f64::MAX } else { t as f64 / g as f64 }
    }

    /// Баланс T/G (0.0-1.0, где 0.5 идеально)
    pub fn tg_balance_score(&self) -> f64 {
        let (t, g) = self.tg_counts();
        let total = t + g;
        if total == 0 { return 0.5; }
        let ratio = t as f64 / total as f64;
        1.0 - (ratio - 0.5).abs() * 2.0
    }

    /// Рекомендуемый поворот на основе T/G
    pub fn suggested_rotation(&self) -> DynamicRotation {
        let signal = self.rna_signal();
        if signal > 1.5 {
            DynamicRotation::Rot0   // много T → активный режим
        } else if signal > 0.8 {
            DynamicRotation::Rot90  // баланс → processing
        } else if signal < 0.5 {
            DynamicRotation::Rot180 // много G → хранение
        } else {
            DynamicRotation::Rot270 // мутация
        }
    }

    /// Архивная сигнатура для multi-chain selection
    pub fn archival_score(&self) -> f64 {
        let (_, g) = self.tg_counts();
        let g_ratio = g as f64 / GENOME_SIZE as f64;
        let consciousness_factor = self.consciousness as f64 / 1000.0;
        g_ratio * 0.5 + consciousness_factor * 0.5
    }

    // ═══════════════════════════════════════════════════════════════
    // BIOLOGICAL METRICS
    // ═══════════════════════════════════════════════════════════════

    pub fn gc_content(&self) -> f64 {
        let gc = self.data.iter()
            .filter(|&&t| matches!(t, Tetrad::G | Tetrad::C))
            .count();
        gc as f64 / GENOME_SIZE as f64
    }

    pub fn complexity(&self) -> f64 {
        let mut transitions = 0;
        for i in 1..GENOME_SIZE {
            if self.data[i] != self.data[i - 1] {
                transitions += 1;
            }
        }
        transitions as f64 / (GENOME_SIZE - 1) as f64
    }

    pub fn biological_age(&self) -> f64 {
        1.0 - (self.telomere_length as f64 / TELOMERE_MAX as f64)
    }

    /// Деление клетки — сокращает теломеры
    pub fn divide(&mut self) -> bool {
        if self.telomere_length < 100 || self.division_count >= HAYFLICK_LIMIT {
            return false; // Сенесценция
        }
        
        let loss = rand::thread_rng().gen_range(50..150);
        self.telomere_length = self.telomere_length.saturating_sub(loss);
        self.division_count += 1;
        true
    }

    /// Активация теломеразы — бессмертие
    pub fn activate_telomerase(&mut self) {
        self.telomere_length = TELOMERE_MAX;
        self.division_count = 0;
    }

    pub fn increment_mutations(&mut self) {
        self.mutations += 1;
    }

    // ═══════════════════════════════════════════════════════════════
    // CRISPR EDITING
    // ═══════════════════════════════════════════════════════════════

    pub fn crispr_splice(&mut self, position: usize, new_base: Tetrad) {
        if position < GENOME_SIZE {
            self.data[position] = new_base;
            self.mutations += 1;
            self.rehash();
            self.calculate_consciousness();
        }
    }

    pub fn crispr_join(&mut self, pos1: usize, pos2: usize) {
        if pos1 < GENOME_SIZE && pos2 < GENOME_SIZE {
            self.data.swap(pos1, pos2);
            self.mutations += 1;
            self.rehash();
            self.calculate_consciousness();
        }
    }

    pub fn crispr_delete(&mut self, position: usize) {
        if position < GENOME_SIZE {
            self.data[position] = Tetrad::random();
            self.mutations += 1;
            self.rehash();
            self.calculate_consciousness();
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// GENOME BUILDER
// ═══════════════════════════════════════════════════════════════

pub struct GenomeBuilder {
    data: [Tetrad; GENOME_SIZE],
    p53_copies: u8,
    telomere_length: u16,
}

impl GenomeBuilder {
    pub fn new() -> Self {
        Self {
            data: [Tetrad::A; GENOME_SIZE],
            p53_copies: 20,
            telomere_length: TELOMERE_MAX,
        }
    }

    pub fn random() -> Self {
        let mut data = [Tetrad::A; GENOME_SIZE];
        for i in 0..GENOME_SIZE {
            data[i] = Tetrad::random();
        }
        Self {
            data,
            p53_copies: 20,
            telomere_length: TELOMERE_MAX,
        }
    }

    pub fn from_dna(dna: &str) -> Option<Self> {
        if dna.len() != GENOME_SIZE {
            return None;
        }
        let mut data = [Tetrad::A; GENOME_SIZE];
        for (i, c) in dna.chars().enumerate() {
            data[i] = Tetrad::from_char(c)?;
        }
        Some(Self {
            data,
            p53_copies: 20,
            telomere_length: TELOMERE_MAX,
        })
    }

    pub fn p53_copies(mut self, copies: u8) -> Self {
        self.p53_copies = copies;
        self
    }

    pub fn telomere_length(mut self, length: u16) -> Self {
        self.telomere_length = length;
        self
    }

    pub fn elephant_mode(self) -> Self {
        self.p53_copies(20)
    }

    pub fn whale_mode(self) -> Self {
        self.p53_copies(40)
    }

    pub fn build<R: Rotation>(self) -> Genome<R> {
        let mut genome = Genome::<R>::new(self.data);
        genome.p53_copies = self.p53_copies;
        genome.telomere_length = self.telomere_length;
        genome.calculate_consciousness();
        genome
    }

    pub fn build_storage(self) -> Genome<Rot180> {
        self.build()
    }

    pub fn build_active(self) -> Genome<Rot0> {
        self.build()
    }

    pub fn build_mutation(self) -> Genome<Rot270> {
        self.build()
    }
}

impl Default for GenomeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub fn hash_genome_dna(dna: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(dna.as_bytes());
    hasher.finalize().into()
}
