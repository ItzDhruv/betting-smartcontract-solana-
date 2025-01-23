pub mod constants;
pub mod error;
pub mod states;

use anchor_lang::prelude::*;
use states::{create_bet, join_bet, end_bet, announce_winner};

declare_id!("GvAaQZ1HF7BVr3aMiJ3TmuKb4daQCcgUnNoFQ2kp4jF9");

#[program]
pub mod betting_dapp {
    use super::*;

    pub fn create_bet(
        ctx: Context<CreateBet>,
        bet_id: String,
        bet_amount: u64,
        crypto_type: String,
        bet_duration: u64,
    ) -> Result<()> {
        create_bet::create_bet(ctx, bet_id, bet_amount, crypto_type, bet_duration)
    }

    pub fn join_bet(ctx: Context<JoinBet>, participant: Pubkey, fee_amount: u64) -> Result<()> {
        join_bet::join_bet(ctx, participant, fee_amount)
    }

    pub fn end_bet(ctx: Context<EndBet>) -> Result<()> {
        end_bet::end_bet(ctx)
    }

    pub fn announce_winner(ctx: Context<AnnounceWinner>, random_seed: u64) -> Result<()> {
        announce_winner::announce_winner(ctx, random_seed)
    }
}
