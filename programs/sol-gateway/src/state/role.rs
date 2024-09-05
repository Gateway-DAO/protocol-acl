use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum AddressType {
    Wallet,
    // Nft,
    Collection,
}

impl AddressType {
    pub fn to_string(&self) -> String {
        match self {
            AddressType::Wallet => "Wallet",
            // AddressType::Nft => "Nft",
            AddressType::Collection => "Collection",
        }
        .to_string()
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum RoleType {
    View,
    Update,
    Delete,
    Share,
}

impl RoleType {
    pub fn to_string(&self) -> String {
        match self {
            RoleType::View => "V",
            RoleType::Update => "U",
            RoleType::Delete => "D",
            RoleType::Share => "S",
        }
        .to_string()
    }
}
#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct AssignRoleData {
    pub address: Option<Pubkey>,
    pub roles: Vec<RoleType>,
    pub address_type: AddressType,
    pub expires_at: Option<i64>,
}

#[account]
pub struct Role {
    pub file_id: Pubkey,
    pub address: Option<Pubkey>,
    pub roles: Vec<RoleType>,
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
