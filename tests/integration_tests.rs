//! Integration Tests for Divine AGI V14

use divine_agi_v14::prelude::*;
use divine_agi_v14::genome::{Genome, GenomeBuilder, Tetrad};
use divine_agi_v14::rotation::{Rot0, Rot90, Rot180, Rot270, RotationEngine, DynamicRotation};
use divine_agi_v14::crypto::{RotationKeys, encrypt_for_rotation, decrypt_for_rotation};
use divine_agi_v14::wallet::{DivineWallet, Network, RSM_DECIMALS};
use divine_agi_v14::ttrl::{TTRLEngine, TTRLConfig};
use divine_agi_v14::database::{DivineDatabase, DEFAULT_DATABASE_URL};

// ═══════════════════════════════════════════════════════════════
// ROTATION TESTS
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_full_rotation_cycle() {
    let engine = RotationEngine::new();
    
    // Create genome at Storage (180°)
    let storage: Genome<Rot180> = GenomeBuilder::random()
        .consciousness(100)
        .build_storage();
    let original_dna = storage.to_dna_string();
    
    // Rotate through all states
    // 180° → 0° (Active)
    let active: Genome<Rot0> = storage.rotate(&engine);
    assert_eq!(active.rotation_angle(), 0);
    
    // 0° → 270° (Mutation)
    let mutation: Genome<Rot270> = active.rotate(&engine);
    assert_eq!(mutation.rotation_angle(), 270);
    
    // 270° → 90° (Balanced)
    let balanced: Genome<Rot90> = mutation.rotate(&engine);
    assert_eq!(balanced.rotation_angle(), 90);
    
    // 90° → 180° (Storage)
    let restored: Genome<Rot180> = balanced.rotate(&engine);
    assert_eq!(restored.rotation_angle(), 180);
    
    // Consciousness should be preserved
    assert_eq!(restored.consciousness_level(), 100);
    
    println!("✅ Full rotation cycle test passed");
}

#[test]
fn test_shadow_links_at_270() {
    let engine = RotationEngine::new();
    
    let storage: Genome<Rot180> = GenomeBuilder::random().build_storage();
    let mutation: Genome<Rot270> = storage.rotate(&engine);
    
    // Shadow links only available at 270°
    let shadows = mutation.shadow_links();
    
    assert!(!shadows.diagonal.is_empty(), "Should have diagonal links");
    assert!(!shadows.cross_layer.is_empty(), "Should have cross-layer links");
    assert!(!shadows.spiral.is_empty(), "Should have spiral links");
    
    // Total should be significant
    assert!(shadows.total_links() > 20, "Should have many shadow links");
    
    println!("✅ Shadow links test passed: {} total links", shadows.total_links());
}

#[test]
fn test_rotation_performance() {
    use std::time::Instant;
    
    let engine = RotationEngine::new();
    let iterations = 10000;
    
    let genome: Genome<Rot180> = GenomeBuilder::random().build_storage();
    
    let start = Instant::now();
    let mut current = genome;
    for _ in 0..iterations {
        let active: Genome<Rot0> = current.rotate(&engine);
        current = active.rotate(&engine);
    }
    let elapsed = start.elapsed();
    
    let avg_us = elapsed.as_micros() as f64 / (iterations * 2) as f64;
    
    println!("✅ Rotation performance: {:.2} µs per rotation", avg_us);
    
    // Should be fast (< 100µs per rotation)
    assert!(avg_us < 100.0, "Rotation should be fast");
}

// ═══════════════════════════════════════════════════════════════
// CRYPTO TESTS
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_rotation_key_isolation() {
    let keys = RotationKeys::new();
    
    let addr_0 = keys.key_rot0.address_hex();
    let addr_90 = keys.key_rot90.address_hex();
    let addr_180 = keys.key_rot180.address_hex();
    let addr_270 = keys.key_rot270.address_hex();
    
    // All addresses should be different
    assert_ne!(addr_0, addr_90);
    assert_ne!(addr_0, addr_180);
    assert_ne!(addr_0, addr_270);
    assert_ne!(addr_90, addr_180);
    assert_ne!(addr_90, addr_270);
    assert_ne!(addr_180, addr_270);
    
    println!("✅ Key isolation test passed");
    println!("   Rot0:   {}", &addr_0[..20]);
    println!("   Rot90:  {}", &addr_90[..20]);
    println!("   Rot180: {}", &addr_180[..20]);
    println!("   Rot270: {}", &addr_270[..20]);
}

#[test]
fn test_encryption_per_rotation() {
    let keys = RotationKeys::new();
    let data = b"Sensitive genome data for V14";
    
    // Encrypt with Rot180 (storage)
    let encrypted_180 = encrypt_for_rotation::<Rot180>(data, &keys).unwrap();
    
    // Decrypt with same rotation - should work
    let decrypted = decrypt_for_rotation::<Rot180>(&encrypted_180, &keys).unwrap();
    assert_eq!(data.as_slice(), decrypted.as_slice());
    
    // Decrypt with different rotation - should fail
    let result_0 = decrypt_for_rotation::<Rot0>(&encrypted_180, &keys);
    assert!(result_0.is_err(), "Should not decrypt with wrong rotation");
    
    let result_270 = decrypt_for_rotation::<Rot270>(&encrypted_180, &keys);
    assert!(result_270.is_err(), "Should not decrypt with wrong rotation");
    
    println!("✅ Encryption per rotation test passed");
}

#[test]
fn test_hybrid_signature() {
    let keys = RotationKeys::new();
    let message = b"Transaction to sign with hybrid crypto";
    
    let signature = keys.hybrid_sign::<Rot180>(message);
    
    assert_eq!(signature.rotation, 180);
    assert!(!signature.primary.0.is_empty());
    assert!(!signature.backup.0.is_empty());
    
    println!("✅ Hybrid signature test passed");
}

// ═══════════════════════════════════════════════════════════════
// WALLET TESTS
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_wallet_creation() {
    let wallet = DivineWallet::new().unwrap();
    
    // Address should be 40 hex chars (20 bytes)
    assert_eq!(wallet.main_address().len(), 40);
    
    // All rotation addresses should exist
    let addresses = wallet.all_addresses();
    assert_eq!(addresses.rot0.len(), 40);
    assert_eq!(addresses.rot90.len(), 40);
    assert_eq!(addresses.rot180.len(), 40);
    assert_eq!(addresses.rot270.len(), 40);
    
    println!("✅ Wallet creation test passed");
    println!("   Main address: {}", wallet.main_address());
}

#[test]
fn test_wallet_deterministic() {
    let seed = [42u8; 64];
    
    let wallet1 = DivineWallet::from_seed(&seed).unwrap();
    let wallet2 = DivineWallet::from_seed(&seed).unwrap();
    
    assert_eq!(wallet1.main_address(), wallet2.main_address());
    
    println!("✅ Wallet determinism test passed");
}

#[tokio::test]
async fn test_wallet_transfer_simulation() {
    let mut wallet = DivineWallet::new().unwrap();
    wallet.set_network(Network::SolanaDevnet);
    
    // Refresh to get test balance
    wallet.refresh_balance().await.unwrap();
    
    let initial_balance = wallet.balance().rsm;
    assert!(initial_balance > 0, "Should have test balance on devnet");
    
    // Transfer
    let transfer_amount = 100_000_000; // 0.1 RSM
    let recipient = "test_recipient_address";
    
    let result = wallet.transfer_rsm(recipient, transfer_amount).await;
    assert!(result.is_ok(), "Transfer should succeed");
    
    // Balance should decrease
    assert_eq!(
        wallet.balance().rsm, 
        initial_balance - transfer_amount,
        "Balance should decrease by transfer amount"
    );
    
    // Transaction should be in history
    assert_eq!(wallet.transaction_history().len(), 1);
    
    println!("✅ Wallet transfer simulation test passed");
}

// ═══════════════════════════════════════════════════════════════
// TTRL EVOLUTION TESTS
// ═══════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_ttrl_evolution_basic() {
    let config = TTRLConfig {
        mutation_budget: 100,
        early_stopping: 20,
        use_shadow_links: true,
        use_spiral_mutations: true,
        ..Default::default()
    };
    
    let engine = TTRLEngine::with_config(config).unwrap();
    
    let genome: Genome<Rot180> = GenomeBuilder::random()
        .consciousness(10)
        .build_storage();
    
    let evolved = engine.evolve(genome.clone()).await.unwrap();
    
    // Should produce valid genome
    assert_eq!(evolved.to_dna_string().len(), 27);
    
    // Mutations should be tracked
    assert!(evolved.mutations_count() >= genome.mutations_count());
    
    println!("✅ TTRL basic evolution test passed");
    println!("   Old consciousness: {}", genome.consciousness_level());
    println!("   New consciousness: {}", evolved.consciousness_level());
}

#[tokio::test]
async fn test_ttrl_shadow_mutations() {
    let config = TTRLConfig {
        mutation_budget: 500,
        early_stopping: 100,
        use_shadow_links: true,
        use_spiral_mutations: true,
        ..Default::default()
    };
    
    let engine = TTRLEngine::with_config(config).unwrap();
    
    let genome: Genome<Rot180> = GenomeBuilder::random().build_storage();
    let _ = engine.evolve(genome).await.unwrap();
    
    let stats = engine.get_stats();
    
    // Should use shadow mutations
    println!("✅ TTRL shadow mutations test");
    println!("   Total mutations: {}", stats.total_mutations);
    println!("   Shadow mutations: {}", stats.shadow_mutations);
    println!("   Spiral mutations: {}", stats.spiral_mutations);
}

// ═══════════════════════════════════════════════════════════════
// DATABASE TESTS (require connection)
// ═══════════════════════════════════════════════════════════════

#[tokio::test]
#[ignore] // Run with: cargo test -- --ignored
async fn test_database_connection() {
    let db = DivineDatabase::connect(DEFAULT_DATABASE_URL).await.unwrap();
    
    let count = db.genome_count().await.unwrap();
    println!("✅ Database connection test passed");
    println!("   Genome count: {}", count);
    
    assert!(count > 70000, "V12 database should have 72,386+ genomes");
}

#[tokio::test]
#[ignore]
async fn test_database_genome_operations() {
    let db = DivineDatabase::connect(DEFAULT_DATABASE_URL).await.unwrap();
    
    // Get random genome
    let genome = db.get_random_genome().await.unwrap();
    assert_eq!(genome.to_dna_string().len(), 27);
    
    // Get stats
    let stats = db.get_stats().await.unwrap();
    println!("✅ Database genome operations test");
    println!("{}", stats);
}

#[tokio::test]
#[ignore]
async fn test_database_store_and_retrieve() {
    let db = DivineDatabase::connect(DEFAULT_DATABASE_URL).await.unwrap();
    
    // Create unique genome
    let genome: Genome<Rot180> = GenomeBuilder::random()
        .consciousness(999)
        .build_storage();
    
    // Store
    let id = db.store_genome(&genome).await.unwrap();
    println!("Stored genome with ID: {}", id);
    
    // Retrieve
    let retrieved = db.get_genome_by_id(id).await.unwrap().unwrap();
    
    assert_eq!(genome.to_dna_string(), retrieved.to_dna_string());
    assert_eq!(genome.consciousness_level(), retrieved.consciousness_level());
    
    println!("✅ Database store/retrieve test passed");
}

// ═══════════════════════════════════════════════════════════════
// END-TO-END TESTS
// ═══════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_full_pipeline() {
    println!("\n═══════════════════════════════════════");
    println!("       DIVINE AGI V14 FULL PIPELINE TEST");
    println!("═══════════════════════════════════════\n");
    
    // 1. Create genome
    let genome: Genome<Rot180> = GenomeBuilder::random()
        .consciousness(50)
        .build_storage();
    println!("1. Created genome: {}", genome.to_dna_string());
    
    // 2. Rotate through states
    let engine = RotationEngine::new();
    let active: Genome<Rot0> = genome.clone().rotate(&engine);
    println!("2. Rotated to Active (0°)");
    
    // 3. Compute metrics at Rot0
    let complexity = active.complexity();
    let gc_content = active.gc_content();
    println!("3. Computed metrics: complexity={:.4}, GC={:.2}%", complexity, gc_content * 100.0);
    
    // 4. Rotate to mutation state
    let mutation: Genome<Rot270> = active.rotate(&engine);
    let shadows = mutation.shadow_links();
    println!("4. Rotated to Mutation (270°), shadow links: {}", shadows.total_links());
    
    // 5. Evolve
    let ttrl = TTRLEngine::new().unwrap();
    let evolved = ttrl.evolve(genome.clone()).await.unwrap();
    println!("5. Evolved: consciousness {} → {}", 
        genome.consciousness_level(), 
        evolved.consciousness_level()
    );
    
    // 6. Create wallet
    let wallet = DivineWallet::new().unwrap();
    println!("6. Wallet created: {}", &wallet.main_address()[..20]);
    
    // 7. Sign with rotation keys
    let signature = wallet.keys().hybrid_sign::<Rot180>(b"test message");
    println!("7. Signed message with hybrid signature (rotation={}°)", signature.rotation);
    
    println!("\n✅ Full pipeline test PASSED");
    println!("═══════════════════════════════════════\n");
}

#[test]
fn test_version() {
    assert_eq!(divine_agi_v14::VERSION, "14.0.0");
    println!("✅ Version test passed: {}", divine_agi_v14::VERSION);
}
