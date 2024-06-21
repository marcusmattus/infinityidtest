use anchor_lang::prelude::*;
use crate::{state::*, error::ErrorCode};

#[derive(Accounts)]
pub struct EarnPoints<'info> {
    #[account(mut,
        constraint = user.key() == profile.authority @ ErrorCode::InvalidUser    //check whether it is authorized user or not and then add points to an authorized user.
    )]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"create_profile", user.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, Profile>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone)]
pub struct EarnPointsParams {
    pub points: u64
}

pub fn earn_points(ctx: Context<EarnPoints>, params: &EarnPointsParams) -> Result<()> {
    let profile = &mut ctx.accounts.profile;
    profile.points += params.points;
    Ok(())
}
