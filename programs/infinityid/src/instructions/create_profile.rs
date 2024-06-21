use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init, 
        payer = user, 
        space = Profile::LEN,
        seeds = [b"create_profile", user.key().as_ref()],
        bump
    )]
    pub profile: Box<Account<'info, Profile>>,
    
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CreateProfileParams {
    pub username: String
}

pub fn create_profile(ctx: Context<CreateProfile>, params: &CreateProfileParams) -> Result<()> {
    let profile = ctx.accounts.profile.as_mut();
    profile.authority = ctx.accounts.user.key();
    profile.username = params.username.clone();
    profile.points = 0;
    Ok(())
}