use crate::state::RoleType;
use crate::Errors;
use anchor_lang::prelude::*;

pub fn valid_rule(roles: &Vec<RoleType>) -> bool {
    if roles.is_empty() || roles.len() > 16 {
        return false;
    }
    for role in roles {
        if !role.is_valid() {
            return false;
        }
    }
    true
}

pub fn valid_rules(roles: &Vec<RoleType>, resource: &String, permission: &String) -> bool {
    if !valid_rule(roles) {
        return false;
    }
    for item in vec![resource, permission].iter() {
        if !valid_rule_string(item) {
            return false;
        }
    }
    true
}

pub fn valid_rule_string(text: &String) -> bool {
    if text.is_empty() || text.as_bytes().len() > 16 {
        return false;
    }
    for char in text.chars() {
        if !char.is_ascii_alphanumeric() {
            if char == '*' && text.as_bytes().len() == 1 {
                continue;
            }
            return false;
        }
    }
    true
}

pub fn allowed_perm(rule1: &String, rule2: &String) -> bool {
    if rule1 == rule2 || rule2 == "*" {
        return true;
    }

    false
}

pub fn validate_ns_permission(namespace: &String) -> Result<()> {
    if namespace != &"*" {
        if let Err(_) = namespace.parse::<u8>() {
            return err!(Errors::InvalidNamespace);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::role::RoleType;

    #[test]
    fn test_valid_rules() {
        assert_eq!(
            valid_rules(
                &vec![RoleType::View],
                &"resource".to_string(),
                &"permission".to_string()
            ),
            true
        );
        // Empty Role, Resource or Permission are not allowed.
        assert_eq!(
            valid_rules(&vec![], &"resource".to_string(), &"permission".to_string()),
            false
        );
        // 16 Characters max per Resource or Permission.
        assert_eq!(
            valid_rules(
                &vec![RoleType::Update],
                &"12345678901234567".to_string(),
                &"permission".to_string()
            ),
            false
        );
        // Only Alphanumeric chars allowed.
        assert_eq!(
            valid_rules(
                &vec![RoleType::Delete],
                &"resource".to_string(),
                &"-".to_string()
            ),
            false
        );
        // Allow "*" on Resource and Permission.
        assert_eq!(
            valid_rules(&vec![RoleType::Share], &"*".to_string(), &"*".to_string()),
            true
        );
    }

    #[test]
    fn test_valid_rule() {
        assert_eq!(valid_rule(&vec![RoleType::View]), true);
        assert_eq!(valid_rule(&vec![]), false);
        assert_eq!(valid_rule(&vec![RoleType::View, RoleType::Update]), true);
    }

    #[test]
    fn test_valid_rule_string() {
        assert_eq!(valid_rule_string(&"resource".to_string()), true);
        assert_eq!(valid_rule_string(&"".to_string()), false);
        assert_eq!(valid_rule_string(&"12345678901234567".to_string()), false);
        assert_eq!(valid_rule_string(&"-".to_string()), false);
        assert_eq!(valid_rule_string(&"*".to_string()), true);
    }

    #[test]
    fn test_allowed_perm() {
        assert_eq!(allowed_perm(&"add".to_string(), &"add".to_string()), true);
        assert_eq!(allowed_perm(&"add".to_string(), &"edit".to_string()), false);
        assert_eq!(allowed_perm(&"add".to_string(), &"*".to_string()), true);
    }

    #[test]
    fn test_validate_ns_permission() {
        assert_eq!(validate_ns_permission(&"*".to_string()), Ok(()));
        assert_eq!(validate_ns_permission(&"0".to_string()), Ok(()));
        assert_eq!(validate_ns_permission(&"1".to_string()), Ok(()));
        assert_eq!(validate_ns_permission(&"255".to_string()), Ok(()));
        assert_eq!(
            validate_ns_permission(&"256".to_string()),
            err!(Errors::InvalidNamespace)
        );
        assert_eq!(
            validate_ns_permission(&"-1".to_string()),
            err!(Errors::InvalidNamespace)
        );
        assert_eq!(
            validate_ns_permission(&"a".to_string()),
            err!(Errors::InvalidNamespace)
        );
    }
}
