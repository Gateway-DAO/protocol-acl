use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum AddressType {
    Wallet,
    // Nft,
    Collection,
    DID,
}

impl AddressType {
    pub fn to_string(&self) -> String {
        match self {
            AddressType::Wallet => "Wallet",
            // AddressType::Nft => "Nft",
            AddressType::Collection => "Collection",
            AddressType::DID => "DID",
        }
        .to_string()
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum ActionType {
    View,
    Update,
    Delete,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct AssignRoleData {
    pub address: Pubkey,
    pub permission_level: u8, // 1 (View), 2 (Update), 3 (Delete)
    pub can_share: bool,
    pub address_type: AddressType,
    pub expires_at: Option<i64>,
}

#[account]
pub struct Role {
    pub file_id: Pubkey,
    pub address: Pubkey,
    pub permission_level: u8, // 1 for View, 2 for Update, 3 for Delete
    pub can_share: bool,
    pub address_type: AddressType,
    pub expires_at: Option<i64>,
    pub bump: u8,
}

#[event]
pub struct RolesChanged {
    pub time: i64,
    #[index]
    pub file_id: Pubkey,
}
