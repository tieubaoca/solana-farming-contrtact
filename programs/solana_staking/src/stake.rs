use anchor_spl::token::{self, Mint, Transfer};

pub use super::*;

pub fn initialize(
    ctx: Context<Initialize>,
    start_ts: i64,
    end_ts: i64,
    total_reward: u64,
) -> Result<()> {
    let clock_ins = Clock::get()?;
    if start_ts >= end_ts {
        return err!(StakeError::InvalidTimestamp);
    }
    if clock_ins.unix_timestamp > start_ts {
        return err!(StakeError::InvalidTimestamp);
    }
    if total_reward == 0 {
        return err!(StakeError::InvalidAmount);
    }
    let payer = &ctx.accounts.user;
    let stake_token_account = &ctx.accounts.stake_token_account;
    let reward_token_account = &ctx.accounts.reward_token_account;
    let stake_pool = &mut ctx.accounts.stake_pool;
    stake_pool.authority = *payer.key;
    stake_pool.total_stake = 0;
    stake_pool.total_reward = total_reward;
    stake_pool.start_ts = start_ts;
    stake_pool.end_ts = end_ts;
    stake_pool.last_update_ts = start_ts;
    stake_pool.accumulate_per_share = 0;
    stake_pool.stake_token_mint = stake_token_account.mint;
    stake_pool.reward_token_mint = reward_token_account.mint;

    Ok(())
}

pub fn init_stake(ctx: Context<InitStake>) -> Result<()> {
    let stake = &mut ctx.accounts.stake;
    let user = &ctx.accounts.user;

    stake.authority = user.key();
    stake.amount = 0;
    stake.reward_debt = 0;

    Ok(())
}

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    if amount == 0 {
        return err!(StakeError::InvalidAmount);
    }
    let stake_pool = &mut ctx.accounts.stake_pool;
    let stake = &mut ctx.accounts.stake;
    let stake_token_account = &ctx.accounts.stake_token_account;
    let reward_token_account = &ctx.accounts.reward_token_account;
    let stake_pool_token_account = &ctx.accounts.stake_pool_token_account;
    let stake_pool_reward_token_account = &ctx.accounts.stake_pool_reward_token_account;
    let token_program = &ctx.accounts.token_program;

    if stake_pool.stake_token_mint != stake_pool_token_account.mint
        || stake_pool.reward_token_mint != stake_pool_reward_token_account.mint
    {
        return err!(StakeError::InvalidAccount);
    }

    update_pool(stake_pool);

    let pending_reward =
        stake.amount * stake_pool.accumulate_per_share / PRECISION_FACTOR - stake.reward_debt;

    stake_pool.total_stake += amount;

    stake.amount += amount;
    stake.reward_debt = stake.amount * stake_pool.accumulate_per_share / PRECISION_FACTOR;

    if pending_reward > 0 {
        let result = token::transfer(
            CpiContext::new(
                token_program.to_account_info(),
                Transfer {
                    from: stake_pool_reward_token_account.to_account_info(),
                    to: reward_token_account.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            ),
            pending_reward,
        );
        if let Err(err) = result {
            return Err(err);
        }
    }

    let result = token::transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: stake_token_account.to_account_info(),
                to: stake_pool_token_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        amount,
    );
    if let Err(err) = result {
        return Err(err);
    }

    Ok(())
}

pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    if amount == 0 {
        return err!(StakeError::InvalidAmount);
    }
    let stake_pool = &mut ctx.accounts.stake_pool;
    let stake = &mut ctx.accounts.stake;
    let stake_token_account = &ctx.accounts.stake_token_account;
    let reward_token_account = &ctx.accounts.reward_token_account;
    let stake_pool_token_account = &ctx.accounts.stake_pool_token_account;
    let stake_pool_reward_token_account = &ctx.accounts.stake_pool_reward_token_account;
    let token_program = &ctx.accounts.token_program;

    update_pool(stake_pool);

    let pending_reward =
        stake.amount * stake_pool.accumulate_per_share / PRECISION_FACTOR - stake.reward_debt;

    stake_pool.total_stake -= amount;

    stake.amount -= amount;
    stake.reward_debt = stake.amount * stake_pool.accumulate_per_share / PRECISION_FACTOR;

    if pending_reward > 0 {
        let result = token::transfer(
            CpiContext::new(
                token_program.to_account_info(),
                Transfer {
                    from: stake_pool_reward_token_account.to_account_info(),
                    to: reward_token_account.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            ),
            pending_reward,
        );
        if let Err(err) = result {
            return Err(err);
        }
    }

    let result = token::transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: stake_pool_token_account.to_account_info(),
                to: stake_token_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        amount,
    );
    if let Err(err) = result {
        return Err(err);
    }

    Ok(())
}

pub fn harvest(ctx: Context<Harvest>) -> Result<()> {
    let stake_pool = &mut ctx.accounts.stake_pool;
    let stake = &mut ctx.accounts.stake;
    let reward_token_account = &ctx.accounts.reward_token_account;
    let stake_pool_reward_token_account = &ctx.accounts.stake_pool_reward_token_account;
    let token_program = &ctx.accounts.token_program;

    update_pool(stake_pool);

    let pending_reward =
        stake.amount * stake_pool.accumulate_per_share / PRECISION_FACTOR - stake.reward_debt;

    stake.reward_debt = stake.amount * stake_pool.accumulate_per_share / PRECISION_FACTOR;

    if pending_reward > 0 {
        let result = token::transfer(
            CpiContext::new(
                token_program.to_account_info(),
                Transfer {
                    from: stake_pool_reward_token_account.to_account_info(),
                    to: reward_token_account.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            ),
            pending_reward,
        );
        if let Err(err) = result {
            return Err(err);
        }
    }

    Ok(())
}

pub fn pending_reward(stake_pool: &StakePool, stake: &Stake) -> u64 {
    let clock_ins = Clock::get().unwrap();
    if clock_ins.unix_timestamp > stake_pool.last_update_ts && stake_pool.total_stake > 0 {
        let multiplier = get_multiplier(
            stake_pool,
            stake_pool.last_update_ts,
            clock_ins.unix_timestamp,
        );
        let token_reward =
            multiplier * stake_pool.total_reward / (stake_pool.end_ts - stake_pool.start_ts) as u64;
        let adjusted_accumulate_per_share = stake_pool.accumulate_per_share
            + token_reward * PRECISION_FACTOR / stake_pool.total_stake;

        stake.amount * adjusted_accumulate_per_share / PRECISION_FACTOR - stake.reward_debt
    } else {
        stake.amount * stake_pool.accumulate_per_share / PRECISION_FACTOR - stake.reward_debt
    }
}

fn update_pool(stake_pool: &mut StakePool) {
    let clock_ins = Clock::get().unwrap();
    if clock_ins.unix_timestamp <= stake_pool.start_ts {
        return;
    }
    if stake_pool.total_stake == 0 {
        stake_pool.last_update_ts = clock_ins.unix_timestamp;
        return;
    }
    let multiplier = get_multiplier(
        stake_pool,
        stake_pool.last_update_ts,
        clock_ins.unix_timestamp,
    );

    let token_reward =
        multiplier * stake_pool.total_reward / (stake_pool.end_ts - stake_pool.start_ts) as u64;

    stake_pool.accumulate_per_share += token_reward * PRECISION_FACTOR / stake_pool.total_stake;
    stake_pool.last_update_ts = clock_ins.unix_timestamp;
}

fn get_multiplier(stake_pool: &StakePool, from: i64, to: i64) -> u64 {
    if to <= stake_pool.end_ts {
        return (to - from) as u64;
    } else if from >= stake_pool.end_ts {
        return 0;
    } else {
        return (stake_pool.end_ts - from) as u64;
    }
}
