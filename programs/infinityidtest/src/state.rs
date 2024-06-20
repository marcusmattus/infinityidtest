use anchor_lang::prelude::*;

#[account]
#[derive(Default, BorshSerialize, BorshDeserialize)]
pub struct Profile {
    pub username: String,
    pub points: u64,
}

impl Profile {
    const LEN: usize = 32 + 8; // Adjust as needed for the actual length of the fields
}
