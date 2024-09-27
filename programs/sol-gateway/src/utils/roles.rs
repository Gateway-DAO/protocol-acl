use crate::{ActionType, Errors, Role};
use anchor_lang::prelude::*;

pub fn address_or_wildcard(address: &Option<Pubkey>) -> &[u8] {
    if address.is_none() {
        return b"*".as_ref();
    }
    address.as_ref().unwrap().as_ref()
}

pub fn perform_action(user_role: &Role, action: ActionType) -> Result<()> {
    match action {
        ActionType::View => {
            if user_role.permission_level >= 1 {
                // Allow viewing
            } else {
                return Err(Errors::InsufficientPermission.into());
            }
        }
        ActionType::Update => {
            if user_role.permission_level >= 2 {
                // Allow updating
            } else {
                return Err(Errors::InsufficientPermission.into());
            }
        }
        ActionType::Delete => {
            if user_role.permission_level >= 3 {
                // Allow deleting
            } else {
                return Err(Errors::InsufficientPermission.into());
            }
        }
    }
    Ok(())
}
