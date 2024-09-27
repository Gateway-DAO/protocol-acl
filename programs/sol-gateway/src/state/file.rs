use crate::state::metadata::*;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Default, Debug)]
pub struct FileData {
    pub id: Pubkey,
    pub recovery: Option<Pubkey>,
    pub fid: String,
    pub size: u64,
    pub checksum: String,
    pub expires_at: i64,
    pub metadata: Option<Vec<Metadata>>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Debug)]
pub struct UpdateFileData {
    pub authority: Pubkey,
    pub recovery: Option<Pubkey>,
    pub fid: String,
    pub fee: Option<u64>,
    pub size: Option<u64>,
    pub checksum: String,
    pub expires_at: Option<i64>,
}

#[account]
pub struct File {
    pub id: Pubkey,
    pub authority: Pubkey,
    pub recovery: Option<Pubkey>,
    pub bump: u8,
    pub fid: String,
    pub roles_updated_at: i64,
    pub rules_updated_at: i64,
    pub fee: Option<u64>,
    pub size: u64,
    pub checksum: String,
    pub expires_at: i64,
}

impl File {
    pub const MAX_SIZE: usize = 162 + 8 + 4 + 32;
}

#[event]
pub struct FileChanged {
    pub time: i64,
    #[index]
    pub file_id: Pubkey,
    pub authority: Pubkey,
}

#[account]
pub struct Seed {
    pub initialized: bool,
}
