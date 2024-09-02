use crate::state::file::*;
use crate::utils::{program_authority_field, utc_now, validate_string_len};
use crate::Errors;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateFile<'info> {
    pub signer: Signer<'info>, // Only current Authority or Recovery key can update the Authority
    #[account(
        mut,
        seeds = [b"file".as_ref(), file.id.key().as_ref()], 
        bump = file.bump,
        constraint = file.authority == signer.key() || (file.recovery.is_some() && file.recovery.unwrap() == signer.key())   @ Errors::UnauthorizedAuthorityUpdate,
    )]
    pub file: Box<Account<'info, File>>,
    pub system_program: Program<'info, System>,
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
