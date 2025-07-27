pub mod state;
pub mod error;
pub mod instructions;

use anchor_lang::prelude::*;

pub use state::*;
pub use instructions::*;

declare_id!("2Cpk3YWB8EQNvjva4PkxqN3EsxYYeep5m7SEXFQHaQpK");

#[program]
pub mod portfolio_rebalancer {
    use super::*;

    pub fn initialize_portfolio(
        ctx: Context<InitializePortfolio>,
        manager: Pubkey,
        rebalance_threshold: u8,
        min_rebalance_interval: i64,
    ) -> Result<()> {
        instructions::initialize_portfolio(ctx, manager, rebalance_threshold, min_rebalance_interval)
    }
    
    pub fn register_strategy(
        ctx: Context<RegisterStrategy>,
        strategy_id: Pubkey,
        protocol_type: ProtocolType,
        initial_balance: u64,
    ) -> Result<()> {
        instructions::register_strategy(ctx, strategy_id, protocol_type, initial_balance)
    }
    
    pub fn update_performance(
        ctx: Context<UpdatePerformance>,
        strategy_id: Pubkey,
        yield_rate: u64,
        volatility_score: u32,
        current_balance: u64,
    ) -> Result<()> {
        instructions::update_performance(ctx, strategy_id, yield_rate, volatility_score, current_balance)
    }
    
    pub fn extract_capital(
        ctx: Context<ExtractCapital>,
        strategy_ids: Vec<Pubkey>,
    ) -> Result<()> {
        instructions::extract_capital(ctx, strategy_ids)
    }

    pub fn execute_ranking_cycle(
        ctx: Context<ExecuteRankingCycle>,
    ) -> Result<()> {
        instructions::execute_ranking_cycle(ctx)
    }
    
    pub fn redistribute_capital(
        ctx: Context<RedistributeCapital>, 
        allocations: Vec<CapitalAllocation>,
    ) -> Result<()> {
        instructions::redistribute_capital(ctx, allocations)
    }
}
