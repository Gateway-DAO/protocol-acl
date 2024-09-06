use crate::state::file::*;
use crate::state::role::Role;
use crate::state::role::RoleType;
use crate::utils::roles::allowed_roles;
use crate::utils::{program_authority_field, utc_now, validate_string_len};
use crate::Errors;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateFile<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"file".as_ref(), file.id.key().as_ref()], 
        bump = file.bump,
        constraint = file.authority == authority.key() || (file.recovery.is_some() && file.recovery.unwrap() == authority.key()) || (role.is_some() && allowed_roles(&role.as_ref().unwrap().roles, &vec![RoleType::Update])) @ Errors::UnauthorizedAuthorityUpdate,
    )]
    pub file: Box<Account<'info, File>>,

    #[account(
        seeds = [authority.key().as_ref(), file.id.key().as_ref()],
        bump = role.bump,
    )]
    pub role: Option<Box<Account<'info, Role>>>,
}

pub fn update_file(ctx: Context<UpdateFile>, file_data: UpdateFileData) -> Result<()> {
    let file = &mut ctx.accounts.file;
    file.authority = file_data.authority;
    file.recovery = file_data.recovery;
    file.name = validate_string_len(&file_data.name, 0, 16)?;
    file.account_type = program_authority_field(
        &file_data.authority,
        file.account_type,
        file_data.account_type,
    )?;
    file.fee = program_authority_field(&file_data.authority, file.fee, file_data.fee)?;
    file.cached = file_data.cached;
    file.size = file_data.size.unwrap_or(file.size);
    file.checksum = validate_string_len(&file_data.checksum, 0, 32)?;
    file.expires_at = file_data.expires_at.unwrap_or(file.expires_at);

    emit!(FileChanged {
        time: utc_now(),
        file_id: ctx.accounts.file.id,
        authority: ctx.accounts.file.authority,
    });
    Ok(())
}
