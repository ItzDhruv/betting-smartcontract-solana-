use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock;
use anchor_lang::solana_program::system_instruction;
use crate::constants::PLATFORM_FEE_PERCENTAGE;

#[account]
pub struct Bet {
    pub bet_id: String,
    pub bet_amount: u64,
    pub crypto_type: String,
    pub bet_duration: u64,
    pub creator: Pubkey,
    pub end_time: u64,
    pub participants: Vec<Pubkey>,
    pub winner: Option<Pubkey>,
    pub is_active: bool,
    pub platform_account: Pubkey, 
}

impl Bet {
    pub fn initialize(
        &mut self,
        bet_id: String,
        bet_amount: u64,
        crypto_type: String,
        bet_duration: u64,
        creator: &Pubkey,
        platform_account: &Pubkey,
    ) -> Result<()> {
        self.bet_id = bet_id;
        self.bet_amount = bet_amount;
        self.crypto_type = crypto_type;
        self.bet_duration = bet_duration;
        self.creator = *creator;
        self.end_time = clock::Clock::get()?.unix_timestamp as u64 + bet_duration;
        self.participants = vec![];
        self.is_active = true;
        self.winner = None;
        self.platform_account = *platform_account;
        Ok(())
    }

    pub fn join_bet(&mut self, participant: Pubkey, fee_amount: u64) -> Result<()> {
        require!(self.is_active, crate::error::CustomError::BetClosed);
        require!(self.end_time > clock::Clock::get()?.unix_timestamp as u64, crate::error::CustomError::BetClosed);
        require!(!self.participants.contains(&participant), crate::error::CustomError::AlreadyJoined);

        let ix = system_instruction::transfer(
            &participant,
            &self.platform_account,
            fee_amount,
        );

        let recent_blockhash = Clock::get()?.unix_timestamp; 
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&participant),
            &[&participant],
            recent_blockhash,
        );

        self.participants.push(participant);
        Ok(())
    }

    pub fn end_bet(&mut self, creator_key: &Pubkey) -> Result<()> {
        require!(self.creator == *creator_key, crate::error::CustomError::NotBetCreator);
        require!(self.end_time <= clock::Clock::get()?.unix_timestamp as u64, crate::error::CustomError::BetOpen);

        self.is_active = false;
        Ok(())
    }

    pub fn creator_announce_winner(&mut self, creator_key: &Pubkey, random_seed: u64) -> Result<()> {
        require!(self.creator == *creator_key, crate::error::CustomError::NotBetCreator);
        require!(self.participants.len() > 0, crate::error::CustomError::NoParticipants);

        let winner_index = (random_seed as usize) % self.participants.len();
        self.winner = Some(self.participants[winner_index]);

        let platform_fee = self.bet_amount * PLATFORM_FEE_PERCENTAGE / 100;
        let winner_amount = self.bet_amount - platform_fee;

        let platform_fee_ix = system_instruction::transfer(
            &self.creator, 
            &self.platform_account, 
            platform_fee,
        );

        let winner_ix = system_instruction::transfer(
            &self.creator, 
            &self.winner.unwrap(), 
            winner_amount,
        );

        let recent_blockhash = Clock::get()?.unix_timestamp; 
        let tx = Transaction::new_signed_with_payer(
            &[platform_fee_ix, winner_ix],
            Some(&self.creator),
            &[&self.creator],
            recent_blockhash,
        );

        Ok(())
    }
}

// Function for creating a bet
pub fn create_bet(
    ctx: Context<CreateBet>,
    bet_id: String,
    bet_amount: u64,
    crypto_type: String,
    bet_duration: u64,
) -> Result<()> {
    let bet = &mut ctx.accounts.bet;
    bet.initialize(bet_id, bet_amount, crypto_type, bet_duration, ctx.accounts.creator.key, &ctx.accounts.platform_account.key)?;
    Ok(())
}

// Function for joining a bet
pub fn join_bet(
    ctx: Context<JoinBet>,
    participant: Pubkey,
    fee_amount: u64,
) -> Result<()> {
    let bet = &mut ctx.accounts.bet;
    bet.join_bet(participant, fee_amount)?;
    Ok(())
}

// Function for ending a bet
pub fn end_bet(ctx: Context<EndBet>) -> Result<()> {
    let bet = &mut ctx.accounts.bet;
    bet.end_bet(ctx.accounts.creator.key)?;
    Ok(())
}

// Function for announcing the winner
pub fn announce_winner(ctx: Context<AnnounceWinner>, random_seed: u64) -> Result<()> {
    let bet = &mut ctx.accounts.bet;
    bet.creator_announce_winner(ctx.accounts.creator.key, random_seed)?;
    Ok(())
}
