use anchor_lang::prelude::*;

declare_id!("CmvvftAiQfC95jwBpCFUBgACjPnCaZSFqNAb3eCJKKNv");

#[program]
pub mod solana_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
