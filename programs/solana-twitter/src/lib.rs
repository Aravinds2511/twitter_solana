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

//tweet account structure
#[account]
pub struct Tweet {
    pub author: Pubkey,
    pub timestamp: i64,
    pub topic: String,
    pub content: String,
}

//constants for sizing properties
const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_TOPIC_LENGTH: usize = 50 * 4;
const MAX_CONTENT_LENGTH: usize = 280 * 4;

//adding const on tweet account to provide total size
impl Tweet {
    const LEN: usize = DISCRIMINATOR_LENGTH
       + PUBLIC_KEY_LENGTH
       + TIMESTAMP_LENGTH
       + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH
       + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH;
}