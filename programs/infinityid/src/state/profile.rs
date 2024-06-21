use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug)]
pub struct Profile {
    pub username: String,
    pub authority: Pubkey,
    pub points: u64,
}

impl Profile {
    pub const LEN: usize = 8 + std::mem::size_of::<Profile>();
}