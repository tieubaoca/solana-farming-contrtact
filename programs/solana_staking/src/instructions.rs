pub use super::*;
use crate::state::{Stake, StakePool};
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
#[instruction()]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8 + 8 + 8 + 8,
        seeds = [b"stake_pool".as_ref()],
        bump
    )]
    pub stake_pool: Account<'info, StakePool>,
    pub stake_token_account: Account<'info, TokenAccount>,
    pub reward_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct InitStake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8 + 8,
        seeds = [b"stake".as_ref(), &user.key.to_bytes()],
        bump
    )]
    pub stake: Account<'info, Stake>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub stake_pool: Account<'info, StakePool>,

    #[account(
        mut,
        seeds = [b"stake".as_ref(), &user.key.to_bytes()],
        bump
    )]
    pub stake: Account<'info, Stake>,

    #[account(mut)]
    pub stake_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub reward_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub stake_pool_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_pool_reward_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub stake_pool: Account<'info, StakePool>,

    #[account(mut,seeds = [b"stake".as_ref(), &user.key.to_bytes()],
    bump)]
    pub stake: Account<'info, Stake>,

    #[account(mut)]
    pub stake_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub reward_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub stake_pool_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_pool_reward_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct Harvest<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub stake_pool: Account<'info, StakePool>,

    #[account(mut,seeds = [b"stake".as_ref(), &user.key.to_bytes()],
    bump)]
    pub stake: Account<'info, Stake>,

    #[account(mut)]
    pub reward_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub stake_pool_reward_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
