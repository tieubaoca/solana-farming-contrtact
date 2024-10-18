use anchor_lang::prelude::*;
pub mod constants;
pub mod errors;
pub mod instructions;
pub mod stake;
pub mod state;

use constants::*;
use errors::*;
use instructions::*;
use stake::*;
use state::*;
declare_id!("CmvvftAiQfC95jwBpCFUBgACjPnCaZSFqNAb3eCJKKNv");

#[program]
pub mod solana_staking {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        start_ts: i64,
        end_ts: i64,
        total_reward: u64,
    ) -> Result<()> {
        stake::initialize(ctx, start_ts, end_ts, total_reward)
    }

    pub fn init_stake(ctx: Context<InitStake>) -> Result<()> {
        stake::init_stake(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        stake::deposit(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        stake::withdraw(ctx, amount)
    }

    pub fn harvest(ctx: Context<Harvest>) -> Result<()> {
        stake::harvest(ctx)
    }
}

// #[derive(Accounts)]
// pub struct Initialize {}
