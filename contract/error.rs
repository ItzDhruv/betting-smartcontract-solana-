use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Bet is already closed.")]
    BetClosed,
    #[msg("Participant already joined.")]
    AlreadyJoined,
    #[msg("Bet is still open.")]
    BetOpen,
    #[msg("No participants in the bet.")]
    NoParticipants,
    #[msg("Only the creator can perform this action.")]
    NotBetCreator,
}