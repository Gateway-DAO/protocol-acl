use crate::state::role::RoleType;
use anchor_lang::prelude::*;

pub fn address_or_wildcard(address: &Option<Pubkey>) -> &[u8] {
    if address.is_none() {
        return b"*".as_ref();
    }
    address.as_ref().unwrap().as_ref()
}

pub fn allowed_roles(roles: &Vec<RoleType>, allowed_roles: &Vec<RoleType>) -> bool {
    for role in roles {
        if allowed_roles.contains(role) {
            return true;
        }
    }
    false
}
