use crate::state::metadata::*;
use anchor_lang::prelude::*;

///  AccountTypes:
///     0 => Basic  (Files with default fees)
///     1 => Free   (Files with no fees)
#[repr(u8)]
pub enum AccountTypes {
    Basic = 0,
    Free = 1,
}

///  CacheUpdated:
///     0 => Roles (When roles change)
///     1 => Rules   (When rules change)
#[repr(u8)]
pub enum CacheUpdated {
    Roles = 0,
    Rules = 1,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Debug)]
pub struct FileData {
    pub id: Pubkey,
    pub recovery: Option<Pubkey>,
    pub name: String,
    pub cached: bool,
    pub size: u64,
    pub checksum: String,
    pub expires_at: i64,
    pub metadata: Option<Vec<Metadata>>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Debug)]
pub struct UpdateFileData {
    pub authority: Pubkey,
    pub recovery: Option<Pubkey>,
    pub name: String,
    pub cached: bool,
    pub fee: Option<u64>,
    pub size: Option<u64>,
    pub checksum: String,
    pub account_type: u8,
    pub expires_at: Option<i64>,
}

#[account]
pub struct File {
    pub id: Pubkey,
    pub authority: Pubkey,
    pub recovery: Option<Pubkey>,
    pub bump: u8,
    pub name: String,
    pub roles_updated_at: i64,
    pub rules_updated_at: i64,
    pub cached: bool,
    pub fee: Option<u64>,
    pub size: u64,
    pub checksum: String,
    pub account_type: u8,
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
