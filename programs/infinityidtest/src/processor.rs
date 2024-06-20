use anchor_lang::prelude::*;
use crate::state::Profile;

pub fn create_profile(ctx: Context<CreateProfile>, username: String) -> ProgramResult {
    let profile = &mut ctx.accounts.profile;
    profile.username = username;
    profile.points = 0;
    Ok(())
}

pub fn earn_points(ctx: Context<EarnPoints>, points: u64) -> ProgramResult {
    let profile = &mut ctx.accounts.profile;
    profile.points += points;
    Ok(())
}
