use crate::Errors;
use anchor_lang::prelude::*;

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
