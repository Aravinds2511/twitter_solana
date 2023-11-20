use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("6V7KDobc2YZaFbQPfmEG3oymuLF9KYN1rCpUHYgAgNnK");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn send_tweet(ctx: Context<SendTweet>, topic: String, content: String) -> Result<()> {
        let tweet: &mut Account<Tweet> = &mut ctx.accounts.tweet;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();

        if topic.chars().count() > 50 {
            return Err(ErrorCode::TopicTooLong.into())
        }

        if content.chars().count() > 280 {
            return Err(ErrorCode::ContentTooLong.into())
        }

        tweet.author = *author.key;
        tweet.timestamp = clock.unix.timestamp;
        tweet.topic = topic;
        tweet.content = content;

        Ok(())
    }
}

//initialisation with needs
#[derive(Accounts)]
pub struct SendTweet<'info> {
    #[account(init, payer = author, space = Tweet::LEN)]
    pub tweet: Account<'info, Tweet>,
    #[account(mut)]
    pub author: Signer<'info>,
    // #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

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

#[error_code]
pub enum ErrorCode {
    #[msg("The provided topic should be 50 characters long maximum.")]
    TopicTooLong,
    #[msg("The provided content should be 280 characters long maximum")]
    ContentTooLong,
}