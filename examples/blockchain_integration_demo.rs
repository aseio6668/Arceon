use anyhow::Result;
use arceon_blockchain::*;
use arceon_core::config::BlockchainConfig;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, warn};
use uuid::Uuid;

/// Demonstration of the integrated blockchain, NFT, and token economy systems
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::init();

    info!("🚀 Starting Arceon Blockchain Integration Demo");

    // Initialize blockchain configuration
    let config = BlockchainConfig {
        network_name: "arceon-testnet".to_string(),
        consensus_algorithm: "pbft".to_string(),
        block_time: Duration::from_secs(10),
        reward_amount: 100,
        max_validators: 21,
        min_stake: 1000,
    };

    // Create blockchain manager with enhanced systems
    let mut blockchain = BlockchainManager::new(&config).await?;
    
    // Start the blockchain in masternode mode
    let node_id = Uuid::new_v4();
    blockchain.start(node_id, true, 10000).await?;

    info!("✅ Blockchain system initialized successfully");

    // Demo 1: NFT Operations
    demo_nft_operations(&mut blockchain).await?;

    // Demo 2: Token Economy Operations
    demo_token_economy(&mut blockchain).await?;

    // Demo 3: DeFi Operations
    demo_defi_operations(&mut blockchain).await?;

    // Demo 4: Cross-chain Bridge
    demo_cross_chain_bridge(&mut blockchain).await?;

    // Demo 5: Comprehensive Stats
    demo_comprehensive_stats(&blockchain).await?;

    info!("🎉 Blockchain Integration Demo completed successfully!");
    Ok(())
}

async fn demo_nft_operations(blockchain: &mut BlockchainManager) -> Result<()> {
    info!("\n=== 🎨 NFT Operations Demo ===");

    let collection_id = "arceon-weapons".to_string();
    let player_id = Uuid::new_v4();

    // Create NFT attributes for a legendary sword
    let attributes = vec![
        NFTAttribute {
            trait_type: "Weapon Type".to_string(),
            value: NFTAttributeValue::String("Sword".to_string()),
            display_type: None,
            rarity: Some(AttributeRarity::Legendary),
        },
        NFTAttribute {
            trait_type: "Attack Power".to_string(),
            value: NFTAttributeValue::Number(850),
            display_type: Some("stat".to_string()),
            rarity: Some(AttributeRarity::Epic),
        },
        NFTAttribute {
            trait_type: "Element".to_string(),
            value: NFTAttributeValue::String("Fire".to_string()),
            display_type: None,
            rarity: Some(AttributeRarity::Rare),
        },
    ];

    // Create blockchain-backed NFT
    let token_id = blockchain.create_blockchain_nft(
        collection_id.clone(),
        player_id,
        attributes
    ).await?;

    info!("🗡️ Created legendary fire sword NFT: {}", token_id);

    // Get NFT system for direct operations
    let nft_system = blockchain.get_nft_system().await;
    let nft_system_read = nft_system.read().await;
    
    // Check NFT details
    if let Some(nft) = nft_system_read.get_nft(token_id).await? {
        info!("📋 NFT Details: {} owned by {}", nft.token_id, nft.owner_id);
        info!("🏆 Evolution Stage: {}", nft.evolution_stage);
    }

    // Get marketplace stats
    let marketplace_stats = nft_system_read.get_marketplace_stats().await?;
    info!("🏪 Marketplace: {} listings, {} sold, {} volume", 
        marketplace_stats.total_listings,
        marketplace_stats.total_sales,
        marketplace_stats.total_volume
    );

    drop(nft_system_read);

    Ok(())
}

async fn demo_token_economy(blockchain: &mut BlockchainManager) -> Result<()> {
    info!("\n=== 💰 Token Economy Demo ===");

    let player1 = Uuid::new_v4();
    let player2 = Uuid::new_v4();

    // Get token economy system
    let token_economy = blockchain.get_token_economy().await;
    let mut token_economy_write = token_economy.write().await;

    // Initialize player wallets
    token_economy_write.create_wallet(player1).await?;
    token_economy_write.create_wallet(player2).await?;

    // Mint some initial tokens for demo
    token_economy_write.mint_tokens(player1, "ARC".to_string(), 10000).await?;
    token_economy_write.mint_tokens(player1, "GOLD".to_string(), 5000).await?;

    info!("💎 Minted 10,000 ARC and 5,000 GOLD tokens for player 1");

    drop(token_economy_write);

    // Transfer tokens through blockchain consensus
    let transaction_id = blockchain.transfer_tokens(
        player1,
        player2,
        "ARC".to_string(),
        1000
    ).await?;

    info!("📤 Transferred 1,000 ARC tokens (Transaction: {})", transaction_id);

    // Check balances
    let token_economy_read = token_economy.read().await;
    let player1_balance = token_economy_read.get_balance(player1, "ARC").await?;
    let player2_balance = token_economy_read.get_balance(player2, "ARC").await?;

    info!("💳 Player 1 ARC balance: {}", player1_balance);
    info!("💳 Player 2 ARC balance: {}", player2_balance);

    Ok(())
}

async fn demo_defi_operations(blockchain: &mut BlockchainManager) -> Result<()> {
    info!("\n=== 🏦 DeFi Operations Demo ===");

    let user_id = Uuid::new_v4();

    // Demo staking operation
    let stake_operation = DeFiOperation::Stake {
        token_type: "ARC".to_string(),
        amount: 5000,
        duration_days: 30,
    };

    let stake_result = blockchain.process_defi_operation(user_id, stake_operation).await?;
    info!("🔒 Staking operation completed: {}", stake_result);

    // Demo liquidity pool operation
    let pool_id = Uuid::new_v4();
    let liquidity_operation = DeFiOperation::AddLiquidity {
        pool_id,
        token_a_type: "ARC".to_string(),
        token_b_type: "GOLD".to_string(),
        token_a_amount: 2000,
        token_b_amount: 1000,
    };

    let liquidity_result = blockchain.process_defi_operation(user_id, liquidity_operation).await?;
    info!("🌊 Liquidity addition completed: {}", liquidity_result);

    // Demo yield farming
    let farm_id = Uuid::new_v4();
    let farming_operation = DeFiOperation::YieldFarm {
        farm_id,
        amount: 3000,
    };

    let farming_result = blockchain.process_defi_operation(user_id, farming_operation).await?;
    info!("🚜 Yield farming started: {}", farming_result);

    Ok(())
}

async fn demo_cross_chain_bridge(blockchain: &mut BlockchainManager) -> Result<()> {
    info!("\n=== 🌉 Cross-Chain Bridge Demo ===");

    let user_id = Uuid::new_v4();

    // Initiate cross-chain transfer
    let bridge_id = blockchain.initiate_cross_chain_bridge(
        user_id,
        "arceon-mainnet".to_string(),
        "ethereum".to_string(),
        "ARC".to_string(),
        2500
    ).await?;

    info!("🔗 Cross-chain bridge initiated: {}", bridge_id);

    // Simulate bridge processing time
    sleep(Duration::from_millis(100)).await;

    // Get token economy to check bridge status
    let token_economy = blockchain.get_token_economy().await;
    let token_economy_read = token_economy.read().await;
    
    let bridge_stats = token_economy_read.get_bridge_stats().await?;
    info!("📊 Bridge Stats: {} total transfers, {} volume", 
        bridge_stats.total_transfers,
        bridge_stats.total_volume
    );

    Ok(())
}

async fn demo_comprehensive_stats(blockchain: &BlockchainManager) -> Result<()> {
    info!("\n=== 📊 Comprehensive Blockchain Stats ===");

    // Get comprehensive economy stats
    let economy_stats = blockchain.get_economy_stats().await?;

    info!("🔗 Blockchain Stats:");
    info!("  - Total Blocks: {}", economy_stats.blockchain_stats.total_blocks);
    info!("  - Last Epoch: {}", economy_stats.blockchain_stats.last_finalized_epoch);
    info!("  - Pending Transactions: {}", economy_stats.blockchain_stats.pending_transactions);

    info!("🎨 NFT Stats:");
    info!("  - Total NFTs: {}", economy_stats.total_nfts);
    info!("  - Collections: {}", economy_stats.total_collections);
    info!("  - 24h Volume: {}", economy_stats.nft_volume_24h);

    info!("💰 Token Economy Stats:");
    info!("  - Total Supply: {}", economy_stats.total_token_supply);
    info!("  - Circulating: {}", economy_stats.circulating_supply);
    info!("  - Total Staked: {}", economy_stats.total_staked);
    info!("  - DeFi TVL: {}", economy_stats.defi_tvl);

    info!("🗳️ Governance Stats:");
    info!("  - Active Proposals: {}", economy_stats.governance_proposals_active);

    info!("🌐 Cross-chain Stats:");
    info!("  - 24h Volume: {}", economy_stats.cross_chain_volume_24h);

    // Get blockchain storage stats
    let blockchain_stats = blockchain.get_blockchain_stats().await;
    info!("📈 Additional Stats:");
    info!("  - Total Players: {}", blockchain_stats.total_players);
    info!("  - Total Areas: {}", blockchain_stats.total_areas);
    info!("  - Total NPCs: {}", blockchain_stats.total_npcs);
    info!("  - World Events: {}", blockchain_stats.total_events);

    Ok(())
}