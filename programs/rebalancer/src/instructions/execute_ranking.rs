use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct ExecuteRankingCycle<'info> {
    #[account(
        mut,
        seeds = [b"portfolio", portfolio.manager.as_ref()],
        bump = portfolio.bump,
        has_one = manager @ ErrorCode::UnauthorizedManager
    )]
    pub portfolio: Account<'info, Portfolio>,
    
    #[account(mut)]
    pub manager: Signer<'info>,
}

pub fn execute_ranking_cycle(
    ctx: Context<ExecuteRankingCycle>,
) -> Result<()> {
    let portfolio = &mut ctx.accounts.portfolio;
    
    // SECURITY VALIDATIONS
    require!(!portfolio.emergency_pause, ErrorCode::EmergencyPaused);
    
    // Check minimum rebalance interval
    let current_timestamp = Clock::get()?.unix_timestamp;
    let time_since_last_rebalance = current_timestamp.saturating_sub(portfolio.last_rebalance);
    
    require!(
        time_since_last_rebalance >= portfolio.min_rebalance_interval,
        ErrorCode::InvalidRebalanceInterval
    );
    
    // UPDATE PORTFOLIO STATE
    portfolio.last_rebalance = current_timestamp;
    
    msg!("Ranking cycle executed at timestamp: {}", current_timestamp);
    msg!("Portfolio has {} total strategies", portfolio.total_strategies);
    
    Ok(())
} 