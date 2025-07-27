use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
    
    #[msg("Rebalance threshold must be between 1-50%")]
    InvalidRebalanceThreshold,
    
    #[msg("Rebalance interval must be between 1 hour and 1 day")]
    InvalidRebalanceInterval,
    
    #[msg("Manager cannot be default pubkey")]
    InvalidManager,
    
    #[msg("Strategy ID cannot be default pubkey")]
    InvalidStrategyId,
    
    #[msg("Yield rate exceeds maximum allowed (500%)")]
    ExcessiveYieldRate,
    
    #[msg("Balance update would cause overflow")]
    BalanceOverflow,
    
    #[msg("Insufficient balance for operation")]
    InsufficientBalance,
    
    #[msg("Portfolio is in emergency pause mode")]
    EmergencyPaused,
    
    #[msg("Strategy not found or invalid")]
    StrategyNotFound,
    
    #[msg("Unauthorized: caller is not portfolio manager")]
    UnauthorizedManager,
    
    #[msg("Invalid volatility score, must be 0-10000")]
    InvalidVolatilityScore,
    
    #[msg("Invalid pool ID")]
    InvalidPoolId,
    
    #[msg("Invalid reserve address")]
    InvalidReserveAddress,
    
    #[msg("Invalid utilization rate")]
    InvalidUtilization,
    
    #[msg("Invalid pair ID")]
    InvalidPairId,
    
    #[msg("Invalid token mint")]
    InvalidTokenMint,
    
    #[msg("Token mints cannot be identical")]
    DuplicateTokenMints,
    
    #[msg("Invalid reward multiplier")]
    InvalidRewardMultiplier,
    
    #[msg("Invalid fee tier")]
    InvalidFeeTier,
    
    #[msg("Invalid validator ID")]
    InvalidValidatorId,
    
    #[msg("Invalid stake pool")]
    InvalidStakePool,
    
    #[msg("Invalid commission rate")]
    InvalidCommission,
    
    #[msg("Invalid unstake delay")]
    InvalidUnstakeDelay,
    
    // AMM and Capital Extraction Errors
    #[msg("Pool state is invalid or corrupted")]
    InvalidPoolState,
    
    #[msg("AMM invariant violation detected")]
    InvariantViolation,
    
    #[msg("Price data is too stale for safe calculations")]
    StalePrice,
    
    #[msg("Invalid or zero price provided")]
    InvalidPrice,
    
    #[msg("Withdrawal amount is too small (dust protection)")]
    WithdrawalTooSmall,
    
    #[msg("Withdrawal amount exceeds safe limits")]
    ExcessiveWithdrawal,
    
    #[msg("Unstake delay period is too long")]
    ExcessiveUnstakeDelay,
    
    #[msg("Protocol utilization too high for safe withdrawal")]
    ProtocolHighUtilization,
    
    // Capital Extraction and Redistribution Errors
    #[msg("Insufficient strategies for operation")]
    InsufficientStrategies,
    
    #[msg("Too many strategies for single operation (max 10)")]
    TooManyStrategies,
    
    #[msg("Invalid protocol type for operation")]
    InvalidProtocolType,
    
    #[msg("Duplicate strategy in allocation")]
    DuplicateStrategy,
    
    #[msg("Invalid performance score for calculation")]
    InvalidPerformanceScore,
}
