use anchor_lang::prelude::*;

/**
 * Types
 */

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Metadata {
    pub key: String,
    pub value: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct MetadataData {
    pub metadata: Vec<Metadata>,
}

/**
 * Accounts
 */

#[account]
pub struct FileMetadata {
    pub file_id: Pubkey,
    pub metadata: Vec<Metadata>,
    pub bump: u8,
}

/**
 * Events
 */

#[event]
pub struct MetadataChanged {
    pub time: i64,
    #[index]
    pub metadata_id: Pubkey,
}
