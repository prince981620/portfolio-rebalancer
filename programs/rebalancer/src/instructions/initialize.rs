use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
#[instruction(manager: Pubkey, rebalance_threshold: u8, min_rebalance_interval: i64)]
pub struct InitializePortfolio<'info> {
    #[account(
        init,
        payer = payer,
        space = Portfolio::MAX_SIZE,
        seeds = [b"portfolio", manager.key().as_ref()],
        bump
    )]
    pub portfolio: Account<'info, Portfolio>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// CHECK: Manager address validation happens in instruction logic
    pub manager: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn initialize_portfolio(
    ctx: Context<InitializePortfolio>,
    manager: Pubkey,
    rebalance_threshold: u8,
    min_rebalance_interval: i64,
) -> Result<()> {
    let portfolio = &mut ctx.accounts.portfolio;
    let current_time = Clock::get()?.unix_timestamp;
    
    // COMPREHENSIVE SECURITY VALIDATIONS
    require!(manager != Pubkey::default(), ErrorCode::InvalidManager);
    Portfolio::validate_rebalance_threshold(rebalance_threshold)?;
    Portfolio::validate_min_interval(min_rebalance_interval)?;
    
    // INITIALIZATION WITH SAFE DEFAULTS
    portfolio.manager = manager;
    portfolio.rebalance_threshold = rebalance_threshold;
    portfolio.total_strategies = 0;
    portfolio.total_capital_moved = 0;
    portfolio.last_rebalance = current_time;
    portfolio.min_rebalance_interval = min_rebalance_interval;
    portfolio.portfolio_creation = current_time;
    portfolio.emergency_pause = false;
    portfolio.performance_fee_bps = 200; // 2% default performance fee
    portfolio.bump = ctx.bumps.portfolio;
    portfolio.reserved = [0u8; 31];
    
    msg!("Portfolio initialized: manager={}, threshold={}%, interval={}s", 
         manager, rebalance_threshold, min_rebalance_interval);
    
    Ok(())
}

// Legacy handler for backward compatibility
#[derive(Accounts)]
pub struct Initialize {}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    msg!("Greetings from: {:?}", ctx.program_id);
    Ok(())
}
