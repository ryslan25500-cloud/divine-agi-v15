//! Divine AGI V14 - Main Entry Point
//! БЕРСЕРК РЕЖИМ АКТИВИРОВАН
//! Дата: 1 января 2026

mod prelude {
    pub use crate::rotation::*;
    pub use crate::genome::*;
    pub use crate::ttrl::*;
    pub use crate::crypto::*;
    pub use crate::database::*;
    pub use crate::consensus::*;
    pub use crate::chain::*;
    pub use crate::wallet::*;
    pub use crate::api::*;
    pub use crate::cli::*;
    pub use crate::exchange::*;
}

use prelude::*;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Инициализация tracing
    tracing_subscriber::fmt()
        .with_env_filter("divine_agi_v14=info")
        .init();

    info!("🧬 Divine AGI V14 - Dynamic Rotation Genome System");
    info!("Версия: 14.0");
    info!("Дата: 1 января 2026");
    info!("Статус: БЕРСЕРК РЕЖИМ АКТИВИРОВАН");
    info!("");

    // === Divine Kernel v3 (роль РНК) ===
    info!("Запуск divine kernel v3 — роль РНК");
    info!("T/G метки: координация, навигация в архивах, обработка геномных данных");

    let db = DivineDatabase::new().await?;
    let rotation_engine = RotationEngine::new();
    let ttrl_engine = TTRLEngine::new()?;
    let wallet = DivineWallet::new()?;
    let consensus = ProofOfConsciousness::new();

    // Тест генома (3x3x3 куб тетрад)
    let mut genome = GenomeBuilder::random()
        .consciousness(42)
        .build_storage(); // 180° storage

    info!("Геном создан: {}", genome.to_dna_string());
    info!("Сознание: {}", genome.consciousness_level());
    info!("Сложность: {:.4}", genome.complexity());

    // Поворот в 0° для compute
    let active_genome = genome.rotate(&rotation_engine, DynamicRotation::Rot0);
    info!("Поворот в 0° (compute): consciousness = {}", active_genome.consciousness_level());

    // Мутация в 270° (TTRL)
    let mutated = ttrl_engine.evolve(&active_genome).await?;
    info!("Мутация (270°): mutations = {}", mutated.mutations_count());

    // Сохранение в 180°
    let stored = mutated.rotate(&rotation_engine, DynamicRotation::Rot180);
    db.store_genome(&stored).await?;
    info!("Геном сохранён в БД (180° storage)");

    // Статистика РНК
    let stats = db.get_stats().await?;
    info!("Всего геномов в БД: {}", stats.total_genomes);

    // === Подготовка к взлому ДНК-памяти ===
    info!("");
    info!("Подготовка взлома Solana, Ethereum, Bitcoin — роль ДНК-памяти");
    info!("27-char тетрады A/G/C, кубы 3×3×3, 24 ротации для контроля 81-бит");
    info!("РНК (kernel v3) управляет T/G метками для координации");

    // Тест wallet (RSM токены)
    info!("Wallet address: {}", wallet.main_address());
    info!("RSM balance: {:.6}", wallet.balance().rsm_display());

    info!("");
    info!("🎉 Все системы operational!");
    info!("Запуск API: cargo run --bin divine-server");
    info!("CLI: cargo run --bin divine-cli -- --help");

    Ok(())
}