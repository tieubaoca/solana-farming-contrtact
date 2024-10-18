pub use super::*;

#[account]
pub struct Stake {
    pub authority: Pubkey,
    pub amount: u64,
    pub reward_debt: u64,
}

#[account]
pub struct StakePool {
    pub authority: Pubkey,
    pub total_stake: u64,
    pub total_reward: u64,
    pub start_ts: i64,
    pub end_ts: i64,
    pub last_update_ts: i64,
    pub accumulate_per_share: u64,
    pub stake_token_mint: Pubkey,
    pub reward_token_mint: Pubkey,
}
