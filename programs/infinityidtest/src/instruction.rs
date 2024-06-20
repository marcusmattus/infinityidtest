use anchor_lang::prelude::*;
use crate::state::Profile;

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8)]
    pub profile: Account<'info, crate::state::Profile>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EarnPoints<'info> {
    #[account(mut)]
    pub profile: Account<'info, crate::state::Profile>,
    pub user: Signer<'info>,
}
