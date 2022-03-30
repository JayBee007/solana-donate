use anchor_lang::prelude::*;
// use anchor_lang::solana_program::{system_instruction, program::{ invoke }};

declare_id!("52T96EnBuLZpkEzabXggxzU4DLDA44xpTsPvdHfu6ogk");

const DISCRIMINATOR_LENGTH: usize = 8;
const TOTAL_DONATION:usize = 8;
const BUMP:usize = 1;
const DONATION_SIZE:usize = 8;
const DONATION_TIMES:usize = 10;


#[program]
pub mod donate {
    use super::*;
   
    pub fn create_program_address(ctx: Context<CreateUserDonation>) -> Result<()> {
        let user_donation = &mut ctx.accounts.user_donation;
        user_donation.total = 0;
        user_donation.bump = *ctx.bumps.get("user_donation").unwrap();
        user_donation.donations = [0;10];

        Ok(())
    }


    // pub fn donate_program(ctx: Context<Donation>, _donation_amount: u64) -> Result<()> {
    //     let sender = &ctx.accounts.sender.key();
    //     let user_donation = &ctx.accounts.user_donation.key();

    //     let instruction = system_instruction::transfer(sender, program_id, donation_amount);

    //     invoke(&instruction, &[ctx.accounts.sender.to_account_info(), ctx.accounts.program_account.to_account_info()])?;

    //     Ok(())
    // }
}

#[derive(Accounts)]
pub struct CreateUserDonation<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(
        init,
        payer = sender,
        space = DISCRIMINATOR_LENGTH + TOTAL_DONATION + (DONATION_SIZE * DONATION_TIMES) + BUMP,
        seeds = [b"user-donation", user_donation.key().as_ref()], 
        bump
    )]
    pub user_donation: Account<'info, UserDonation>,
    pub system_program: Program<'info, System>,

}

// #[derive(Accounts)]
// pub struct Donation<'info> {
    // #[account(
    //     init,
    //     payer = sender,
    //     space = DISCRIMINATOR_LENGTH + TOTAL_DONATION + (DONATION_SIZE * DONATION_TIMES) + BUMP,
    //     seeds = [b"user-donation", user_donation.key().as_ref()], 
    //     bump
    // )]
    // pub user_donation: Account<'info, UserDonation>,
//     #[account(mut)]
//     pub sender: Signer<'info>,
//     pub system_program: Program<'info, System>
// }

#[account]
pub struct UserDonation {
    total: u64,
    bump: u8,
    donations: [u64;10]
  }