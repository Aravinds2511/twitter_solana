use anchor_lang::prelude::*;

declare_id!("6V7KDobc2YZaFbQPfmEG3oymuLF9KYN1rCpUHYgAgNnK");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
