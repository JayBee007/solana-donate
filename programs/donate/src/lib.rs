use anchor_lang::prelude::*;
use anchor_lang::solana_program::{system_instruction, program::{ invoke }};

declare_id!("52T96EnBuLZpkEzabXggxzU4DLDA44xpTsPvdHfu6ogk");

#[program]
pub mod donate {
    use super::*;
    
    pub fn donate_program(ctx: Context<Donation>, donation_amount: u64) -> Result<()> {
        let sender = &ctx.accounts.sender.key();
        let program_id = &ctx.accounts.program_account.key();

        let instruction = system_instruction::transfer(sender, program_id, donation_amount);

        invoke(&instruction, &[ctx.accounts.sender.to_account_info(), ctx.accounts.program_account.to_account_info()])?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Donation<'info> {
    #[account(mut)]
    pub program_account: Account<'info, ProgramAccount>,
    #[account(mut)]
    pub sender: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct ProgramAccount {
    pub pub_key: Pubkey
}