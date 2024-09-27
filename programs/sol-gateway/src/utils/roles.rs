use crate::ActionType;
use anchor_lang::prelude::*;

pub fn address_or_wildcard(address: &Option<Pubkey>) -> &[u8] {
    if address.is_none() {
        return b"*".as_ref();
    }
    address.as_ref().unwrap().as_ref()
}

pub fn perform_action(permission_level: &u8, action: ActionType) -> bool {
    match action {
        ActionType::View => {
            if *permission_level >= 1 {
                return true;
            } else {
                return false;
            }
        }
        ActionType::Update => {
            if *permission_level >= 2 {
                return true;
            } else {
                return false;
            }
        }
        ActionType::Delete => {
            if *permission_level >= 3 {
                return true;
            } else {
                return false;
            }
        }
    }
}
