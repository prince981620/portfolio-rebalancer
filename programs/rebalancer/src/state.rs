use anchor_lang::prelude::*;
use crate::error::ErrorCode;

#[account]
#[derive(Debug)]
pub struct Portfolio {
    pub manager: Pubkey,                    // 32 bytes - Portfolio manager authority
    pub rebalance_threshold: u8,            // 1 byte - Bottom % for reallocation (1-50)
    pub total_strategies: u32,              // 4 bytes - Current strategy count
    pub total_capital_moved: u64,           // 8 bytes - Lifetime capital rebalanced (lamports)
    pub last_rebalance: i64,                // 8 bytes - Unix timestamp of last rebalance
    pub min_rebalance_interval: i64,        // 8 bytes - Minimum seconds between rebalances
    pub portfolio_creation: i64,            // 8 bytes - Portfolio creation timestamp
    pub emergency_pause: bool,              // 1 byte - Emergency stop flag
    pub performance_fee_bps: u16,           // 2 bytes - Performance fee in basis points
    pub bump: u8,                           // 1 byte - PDA bump seed
    pub reserved: [u8; 31],                 // 31 bytes - Future expansion buffer
}
// Total: 136 bytes

#[account]
#[derive(Debug)]
pub struct Strategy {
    pub strategy_id: Pubkey,                // 32 bytes - Unique strategy identifier
    pub protocol_type: ProtocolType,        // Variable size - Protocol-specific data
    pub current_balance: u64,               // 8 bytes - Current capital allocated (lamports)
    pub yield_rate: u64,                    // 8 bytes - Annual yield in basis points (0-50000)
    pub volatility_score: u32,              // 4 bytes - Risk metric (0-10000, 100.00% max)
    pub performance_score: u64,             // 8 bytes - Calculated composite score
    pub percentile_rank: u8,                // 1 byte - 0-100 ranking position
    pub last_updated: i64,                  // 8 bytes - Last metric update timestamp
    pub status: StrategyStatus,             // 1 byte - Current strategy status
    pub total_deposits: u64,                // 8 bytes - Lifetime deposits tracking
    pub total_withdrawals: u64,             // 8 bytes - Lifetime withdrawals tracking
    pub creation_time: i64,                 // 8 bytes - Strategy creation timestamp
    pub bump: u8,                           // 1 byte - PDA bump seed
    pub reserved: [u8; 23],                 // 23 bytes - Future expansion
}
// Total: ~144 bytes + protocol_type size

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
pub enum ProtocolType {
    StableLending { 
        pool_id: Pubkey,                    // 32 bytes - Solend pool identifier
        utilization: u16,                   // 2 bytes - Pool utilization in basis points
        reserve_address: Pubkey,            // 32 bytes - Reserve account address
    },  // 66 bytes total
    YieldFarming { 
        pair_id: Pubkey,                    // 32 bytes - Orca pair identifier
        reward_multiplier: u8,              // 1 byte - Reward boost (1-10x)
        token_a_mint: Pubkey,               // 32 bytes - Token A mint address
        token_b_mint: Pubkey,               // 32 bytes - Token B mint address
        fee_tier: u16,                      // 2 bytes - Pool fee in basis points
    },  // 99 bytes total
    LiquidStaking { 
        validator_id: Pubkey,               // 32 bytes - Marinade validator
        commission: u16,                    // 2 bytes - Validator commission (basis points)
        stake_pool: Pubkey,                 // 32 bytes - Stake pool address
        unstake_delay: u32,                 // 4 bytes - Unstaking delay in epochs
    },  // 70 bytes total
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum StrategyStatus {
    Active,      // Normal operation, participates in rebalancing
    Paused,      // Temporarily disabled, no new allocations
    Deprecated,  // Marked for removal, extract capital when possible
}

#[account]
#[derive(Debug)]
pub struct CapitalPosition {
    pub strategy_id: Pubkey,                // 32 bytes - Reference to strategy
    pub token_a_amount: u64,                // 8 bytes - Token A quantity
    pub token_b_amount: u64,                // 8 bytes - Token B quantity (0 for single asset)
    pub lp_tokens: u64,                     // 8 bytes - LP tokens held
    pub platform_controlled_lp: u64,       // 8 bytes - LP tokens under platform control
    pub position_type: PositionType,        // 1 byte - Position classification
    pub entry_price_a: u64,                 // 8 bytes - Entry price token A (6 decimals)
    pub entry_price_b: u64,                 // 8 bytes - Entry price token B (6 decimals)
    pub last_rebalance: i64,                // 8 bytes - Last position update
    pub accrued_fees: u64,                  // 8 bytes - Accumulated fees in position
    pub impermanent_loss: i64,              // 8 bytes - IL tracking (can be negative)
    pub bump: u8,                           // 1 byte - PDA bump seed
    pub reserved: [u8; 15],                 // 15 bytes - Future expansion
}
// Total: 145 bytes

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
pub enum PositionType {
    SingleAsset,
    LiquidityPair,
    StakedPosition,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct CapitalAllocation {
    pub strategy_id: Pubkey,
    pub amount: u64,
    pub allocation_type: AllocationType,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
pub enum AllocationType {
    TopPerformer,
    RiskDiversification,
    ManagerIncentive,
    PlatformFee,
}

impl Portfolio {
    pub const MAX_SIZE: usize = 8 + 136;
    
    pub fn validate_rebalance_threshold(threshold: u8) -> Result<()> {
        require!(threshold >= 1 && threshold <= 50, ErrorCode::InvalidRebalanceThreshold);
        Ok(())
    }
    
    pub fn can_rebalance(&self, current_time: i64) -> bool {
        !self.emergency_pause && 
        current_time >= self.last_rebalance.saturating_add(self.min_rebalance_interval)
    }
    
    pub fn validate_min_interval(interval: i64) -> Result<()> {
        require!(interval >= 3600 && interval <= 86400, ErrorCode::InvalidRebalanceInterval);
        Ok(())
    }
}

impl Strategy {
    pub const MAX_SIZE: usize = 8 + 200; // Account for largest protocol type
    
    pub fn validate_yield_rate(rate: u64) -> Result<()> {
        require!(rate <= 50000, ErrorCode::ExcessiveYieldRate);
        Ok(())
    }
    
    pub fn validate_balance_update(new_balance: u64) -> Result<()> {
        require!(new_balance < u64::MAX / 1000, ErrorCode::BalanceOverflow);
        Ok(())
    }
    
    pub fn validate_volatility_score(score: u32) -> Result<()> {
        require!(score <= 10000, ErrorCode::InvalidVolatilityScore);
        Ok(())
    }
}

impl ProtocolType {
    pub fn validate(&self) -> Result<()> {
        match self {
            ProtocolType::StableLending { pool_id, utilization, reserve_address } => {
                require!(*pool_id != Pubkey::default(), ErrorCode::InvalidPoolId);
                require!(*reserve_address != Pubkey::default(), ErrorCode::InvalidReserveAddress);
                require!(*utilization <= 10000, ErrorCode::InvalidUtilization);
                Ok(())
            },
            ProtocolType::YieldFarming { 
                pair_id, reward_multiplier, token_a_mint, token_b_mint, fee_tier 
            } => {
                require!(*pair_id != Pubkey::default(), ErrorCode::InvalidPairId);
                require!(*token_a_mint != Pubkey::default(), ErrorCode::InvalidTokenMint);
                require!(*token_b_mint != Pubkey::default(), ErrorCode::InvalidTokenMint);
                require!(*token_a_mint != *token_b_mint, ErrorCode::DuplicateTokenMints);
                require!(*reward_multiplier >= 1 && *reward_multiplier <= 10, ErrorCode::InvalidRewardMultiplier);
                require!(*fee_tier <= 1000, ErrorCode::InvalidFeeTier);
                Ok(())
            },
            ProtocolType::LiquidStaking { 
                validator_id, commission, stake_pool, unstake_delay 
            } => {
                require!(*validator_id != Pubkey::default(), ErrorCode::InvalidValidatorId);
                require!(*stake_pool != Pubkey::default(), ErrorCode::InvalidStakePool);
                require!(*commission <= 1000, ErrorCode::InvalidCommission);
                require!(*unstake_delay <= 50, ErrorCode::InvalidUnstakeDelay);
                Ok(())
            },
        }
    }
    
    pub fn get_protocol_name(&self) -> &'static str {
        match self {
            ProtocolType::StableLending { .. } => "Stable Lending",
            ProtocolType::YieldFarming { .. } => "Yield Farming",
            ProtocolType::LiquidStaking { .. } => "Liquid Staking",
        }
    }
    
    pub fn get_expected_tokens(&self) -> Vec<Pubkey> {
        match self {
            ProtocolType::StableLending { reserve_address, .. } => {
                vec![*reserve_address]
            },
            ProtocolType::YieldFarming { token_a_mint, token_b_mint, .. } => {
                vec![*token_a_mint, *token_b_mint]
            },
            ProtocolType::LiquidStaking { stake_pool, .. } => {
                vec![*stake_pool]
            },
        }
    }
    
    pub fn validate_balance_constraints(&self, balance: u64) -> Result<()> {
        match self {
            ProtocolType::StableLending { .. } => {
                // Minimum 0.1 SOL for lending protocols
                require!(balance >= 100_000_000, ErrorCode::InsufficientBalance);
            },
            ProtocolType::YieldFarming { .. } => {
                // Minimum 0.5 SOL for LP positions (gas + slippage)
                require!(balance >= 500_000_000, ErrorCode::InsufficientBalance);
            },
            ProtocolType::LiquidStaking { .. } => {
                // Minimum 1 SOL for staking (epoch requirements)
                require!(balance >= 1_000_000_000, ErrorCode::InsufficientBalance);
            },
        }
        Ok(())
    }
}

impl CapitalPosition {
    pub const MAX_SIZE: usize = 8 + 145;
    
    // AMM-SAFE WITHDRAWAL CALCULATIONS
    pub fn calculate_lp_withdrawal_amounts(
        &self,
        current_reserve_a: u64,
        current_reserve_b: u64,
        total_lp_supply: u64,
        lp_tokens_to_burn: u64,
    ) -> Result<(u64, u64)> {
        // Validate invariant preservation
        require!(lp_tokens_to_burn <= self.lp_tokens, ErrorCode::InsufficientBalance);
        require!(total_lp_supply > 0, ErrorCode::InvalidPoolState);
        
        // SAFE: Use 128-bit arithmetic to prevent overflow
        let token_a_out = (lp_tokens_to_burn as u128 * current_reserve_a as u128)
            .checked_div(total_lp_supply as u128)
            .ok_or(ErrorCode::BalanceOverflow)? as u64;
            
        let token_b_out = (lp_tokens_to_burn as u128 * current_reserve_b as u128)
            .checked_div(total_lp_supply as u128)
            .ok_or(ErrorCode::BalanceOverflow)? as u64;
        
        // VERIFY: x*y=k invariant maintained
        let new_reserve_a = current_reserve_a.saturating_sub(token_a_out);
        let new_reserve_b = current_reserve_b.saturating_sub(token_b_out);
        let new_k = (new_reserve_a as u128).checked_mul(new_reserve_b as u128)
            .ok_or(ErrorCode::BalanceOverflow)?;
        let old_k = (current_reserve_a as u128).checked_mul(current_reserve_b as u128)
            .ok_or(ErrorCode::BalanceOverflow)?;
            
        // Allow small precision loss but prevent large deviations
        require!(new_k >= old_k.saturating_sub(old_k / 10000), ErrorCode::InvariantViolation); // 0.01% tolerance
        
        Ok((token_a_out, token_b_out))
    }
    
    // REAL-TIME IMPERMANENT LOSS CALCULATION
    pub fn calculate_current_impermanent_loss(
        &self,
        current_price_a: u64,  // Oracle price with 6 decimals
        current_price_b: u64,  // Oracle price with 6 decimals
        price_timestamp: i64,  // Oracle timestamp
    ) -> Result<i64> {
        // Validate price freshness (max 60 seconds old)
        let current_time = Clock::get()?.unix_timestamp;
        require!(current_time - price_timestamp <= 60, ErrorCode::StalePrice);
        
        // Prevent division by zero
        require!(self.entry_price_b > 0 && current_price_b > 0, ErrorCode::InvalidPrice);
        
        // Calculate price ratio changes using safe arithmetic
        let entry_ratio = (self.entry_price_a as u128 * 1_000_000u128) / self.entry_price_b as u128;
        let current_ratio = (current_price_a as u128 * 1_000_000u128) / current_price_b as u128;
        
        // IL = 2 * sqrt(price_ratio) / (1 + price_ratio) - 1
        let ratio_change = current_ratio.checked_div(entry_ratio)
            .ok_or(ErrorCode::BalanceOverflow)?;
            
        // Use integer square root for safety
        let sqrt_ratio = sqrt_u128(ratio_change * 1_000_000u128);
        let il_numerator = 2u128 * sqrt_ratio;
        let il_denominator = 1_000_000u128 + ratio_change;
        
        let il_ratio = il_numerator.checked_div(il_denominator)
            .ok_or(ErrorCode::BalanceOverflow)?;
            
        // Convert to signed percentage (can be negative for gains)
        let il_percentage = (il_ratio as i64) - 1_000_000i64; // Subtract 100%
        
        Ok(il_percentage)
    }
    
    // PROTOCOL-AWARE WITHDRAWAL VALIDATION
    pub fn validate_withdrawal_feasibility(
        &self,
        requested_amount: u64,
        protocol_type: &ProtocolType,
    ) -> Result<()> {
        match protocol_type {
            ProtocolType::YieldFarming { .. } => {
                // AMM: Check minimum liquidity requirements
                require!(requested_amount >= 1000, ErrorCode::WithdrawalTooSmall); // Dust protection
                require!(requested_amount <= self.lp_tokens / 2, ErrorCode::ExcessiveWithdrawal); // Max 50% per tx
            },
            ProtocolType::LiquidStaking { unstake_delay, .. } => {
                // Liquid staking: Warn about delays
                require!(*unstake_delay <= 50, ErrorCode::ExcessiveUnstakeDelay);
                require!(requested_amount <= self.platform_controlled_lp, ErrorCode::InsufficientBalance);
            },
            ProtocolType::StableLending { utilization, .. } => {
                // Lending: Check utilization limits
                require!(*utilization < 9500, ErrorCode::ProtocolHighUtilization); // Max 95% utilization
                require!(requested_amount <= self.token_a_amount, ErrorCode::InsufficientBalance);
            },
        }
        Ok(())
    }
}

// MATHEMATICAL SAFETY HELPERS
fn sqrt_u128(x: u128) -> u128 {
    if x == 0 { return 0; }
    let mut sqrt = x / 2;
    let mut temp = (sqrt + x / sqrt) / 2;
    while temp < sqrt {
        sqrt = temp;
        temp = (sqrt + x / sqrt) / 2;
    }
    sqrt
}

    fn sqrt_u64(x: u64) -> Result<u64> {
        Ok(sqrt_u128(x as u128) as u64)
    }
