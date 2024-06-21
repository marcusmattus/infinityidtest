pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("FFhtCBfYK92GZtvvnGp36UBrZSnd38cH4G2M89BAA5QF");

#[program]
pub mod infinityid {
    use super::*;

    pub fn create_profile(ctx: Context<CreateProfile>, params: CreateProfileParams) -> Result<()> {
        instructions::create_profile(ctx, &params)
    }

    pub fn earn_points(ctx: Context<EarnPoints>, params: EarnPointsParams) -> Result<()> {
        instructions::earn_points(ctx, &params)
    }
}