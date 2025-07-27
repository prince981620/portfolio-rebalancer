use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
#[instruction(strategy_id: Pubkey, protocol_type: ProtocolType, initial_balance: u64)]
pub struct RegisterStrategy<'info> {
    #[account(
        mut,
        seeds = [b"portfolio", portfolio.manager.as_ref()],
        bump = portfolio.bump,
        has_one = manager @ ErrorCode::UnauthorizedManager
    )]
    pub portfolio: Account<'info, Portfolio>,
    
    #[account(
        init,
        payer = manager,
        space = Strategy::MAX_SIZE,
        seeds = [b"strategy", portfolio.key().as_ref(), strategy_id.as_ref()],
        bump
    )]
    pub strategy: Account<'info, Strategy>,
    
    #[account(mut)]
    pub manager: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn register_strategy(
    ctx: Context<RegisterStrategy>,
    strategy_id: Pubkey,
    protocol_type: ProtocolType,
    initial_balance: u64,
) -> Result<()> {
    let portfolio = &mut ctx.accounts.portfolio;
    let strategy = &mut ctx.accounts.strategy;
    let current_time = Clock::get()?.unix_timestamp;
    
    // COMPREHENSIVE SECURITY VALIDATIONS
    require!(!portfolio.emergency_pause, ErrorCode::EmergencyPaused);
    require!(strategy_id != Pubkey::default(), ErrorCode::InvalidStrategyId);
    require!(initial_balance > 0, ErrorCode::InsufficientBalance);
    Strategy::validate_balance_update(initial_balance)?;
    
    // PROTOCOL-SPECIFIC VALIDATION
    protocol_type.validate()?;
    protocol_type.validate_balance_constraints(initial_balance)?;
    
    // STRATEGY INITIALIZATION WITH SAFE DEFAULTS
    strategy.strategy_id = strategy_id;
    strategy.protocol_type = protocol_type;
    strategy.current_balance = initial_balance;
    strategy.yield_rate = 0; // Will be updated by performance tracking
    strategy.volatility_score = 5000; // Start with moderate risk (50%)
    strategy.performance_score = 0; // Calculated after first performance update
    strategy.percentile_rank = 50; // Start at median
    strategy.last_updated = current_time;
    strategy.status = StrategyStatus::Active;
    strategy.total_deposits = initial_balance;
    strategy.total_withdrawals = 0;
    strategy.creation_time = current_time;
    strategy.bump = ctx.bumps.strategy;
    strategy.reserved = [0u8; 23];
    
    // UPDATE PORTFOLIO COUNTERS WITH OVERFLOW PROTECTION
    portfolio.total_strategies = portfolio.total_strategies
        .checked_add(1)
        .ok_or(ErrorCode::BalanceOverflow)?;
    
    msg!("Strategy registered: ID={}, Protocol={}, Balance={}", 
         strategy_id, protocol_type.get_protocol_name(), initial_balance);
    
    Ok(())
}
