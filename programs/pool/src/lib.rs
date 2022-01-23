use anchor_lang::prelude::*;
use std::mem::size_of;

use characters_program::UserAccount;

declare_id!("BrTDHy59xLXR4pFimtoG9CBp4ATgKJpmQtv8wZvb5diQ");

const STATE_SEED: &[u8] = b"state_seed";

#[program]
pub mod pool {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, cap: u8) -> ProgramResult {
        let state = &mut ctx.accounts.state;
        state.members = Vec::with_capacity(cap as usize);
        state.cap = cap;
        Ok(())
    }

    pub fn join(ctx: Context<Join>) -> ProgramResult {
        let state = &mut ctx.accounts.state;
        let user_key = ctx.accounts.user_account.key();
        if !state.members.contains(&user_key) {
            state.members.push(user_key);
        }
        Ok(())
    }

    pub fn take(ctx: Context<Take>) -> ProgramResult {
        let state = &mut ctx.accounts.state;
        let user_key = ctx.accounts.user_account.key();
        if let Some(i) = state.members.iter().position(|&k| k == user_key) {
            state.members.remove(i);
        }
        Ok(())
    }

    pub fn close(_ctx: Context<Close>) -> ProgramResult {
        msg!("Pool is closed");
        Ok(())
    }
}

#[account]
pub struct ProgramState {
    pub members: Vec<Pubkey>,
    pub cap: u8,
}

#[derive(Accounts)]
#[instruction(cap: u8)]
pub struct Initialize<'info> {
    #[account(
        init_if_needed,
        payer = payer,
        seeds = [STATE_SEED],
        bump,
        space = 16 + size_of::<Pubkey>() * (cap as usize),
    )]
    pub state: Account<'info, ProgramState>,
    #[account(mut)]
    pub payer: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Join<'info> {
    #[account(
        mut,
        seeds = [STATE_SEED],
        bump,
        constraint =
            state.members.len() < state.cap as usize
            @ ErrorCode::CapacityExceeded,
    )]
    pub state: Account<'info, ProgramState>,
    pub user_account: Account<'info, UserAccount>,
}

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(
        mut,
        seeds = [STATE_SEED],
        bump,
    )]
    pub state: Account<'info, ProgramState>,
    pub user_account: Account<'info, UserAccount>,
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(
        mut,
        close = receiver,
        seeds = [STATE_SEED],
        bump,
    )]
    pub state: Account<'info, ProgramState>,
    #[account(mut)]
    pub receiver: SystemAccount<'info>,
}

#[error]
pub enum ErrorCode {
    #[msg("Pool capacity exceeded.")]
    CapacityExceeded,
}
