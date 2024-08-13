use anchor_lang::error_code;

#[error_code]
pub enum Errors {
    #[msg("Only current Authority or Recovery accounts can update the File authority")]
    UnauthorizedAuthorityUpdate,
    #[msg("Role, Resource or Permission must be betwen 1 and 16 alphanumeric characters long")]
    InvalidRule,
    #[msg("Role must be between 1 and 16 alphanumeric characters long")]
    InvalidRole,
    #[msg("The provided string is too short")]
    StringTooShort,
    #[msg("The provided string is too long")]
    StringTooLong,
    #[msg("The user does not have enough privileges to perform this action")]
    Unauthorized,
    #[msg("The Sol Gateway FILE ID does not match the one defined in the program")]
    InvalidFileID,
    #[msg("Invalid address type, mus be either 'Wallet', 'Nft', 'Collection' or a wildcard '*'")]
    InvalidAddressType,
    #[msg("Invalid namespace, must be either an u8 number (0-255) or a wildcard '*'")]
    InvalidNamespace,
    #[msg("GATEWAY_FILE_ID is missing on lib.rs")]
    MissingSolGatewayFileId,
    #[msg("The Gateway Seed account is missing")]
    MissingSeedAccount,
    #[msg("Only program authority can perform this action")]
    UnauthorizedProgramAuthority,
    #[msg("Insufficient funds for transaction")]
    InsufficientFunds,
    #[msg("The provided file ID does not match the metadata account")]
    InvalidFileId,
    #[msg("Unauthorized metadata update")]
    UnauthorizedMetadataUpdate,
}
