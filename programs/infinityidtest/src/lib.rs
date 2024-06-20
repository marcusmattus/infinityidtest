use anchor_lang::prelude::*;
use anchor_lang::prelude::Account as ProgramAccount;
use anchor_spl::token_interface::spl_token_metadata_interface::borsh::{BorshDeserialize, BorshSerialize};

pub mod state;
pub mod processor;
pub mod errors;
pub mod instruction;

declare_id!("CmEQCf7zfD2SRz74RSgLtjnUkthA4Se4azyYn61958hF");

#[program]
pub mod infinityid {
    use super::*;

    pub fn create_profile(ctx: Context<CreateProfile>, username: String) -> ProgramResult {
        processor::create_profile(ctx, username)
    }

    pub fn earn_points(ctx: Context<EarnPoints>, points: u64) -> ProgramResult {
        processor::earn_points(ctx, points)
    }
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(init, payer = user, space = 8 + Profile::LEN)]
    pub profile: ProgramAccount<'info, Profile>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Eq for CreateProfile<'info> {}

#[derive(Accounts)]
pub struct EarnPoints<'info> {
    #[account(mut)]
    pub profile: ProgramAccount<'info, Profile>,
    #[account(signer)]
    pub user: Signer<'info>,
}

#[account]
#[derive(Default, BorshSerialize, BorshDeserialize)]
pub struct Profile {
    pub username: String,
    pub points: u64,
}

impl Profile {
    const LEN: usize = 32 + 8; // Adjust as needed for the actual length of the fields
}
