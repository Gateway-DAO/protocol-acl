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
    let app = &mut ctx.accounts.file;
    app.authority = file_data.authority;
    app.recovery = file_data.recovery;
    app.name = validate_string_len(&file_data.name, 0, 16)?;
    app.account_type =
        program_authority_field(&file_data.authority, app.account_type, file_data.account_type)?;
    app.fee = program_authority_field(&file_data.authority, app.fee, file_data.fee)?;
    app.cached = file_data.cached;
    app.expires_at =
        program_authority_field(&file_data.authority, app.expires_at, file_data.expires_at)?;
    emit!(FileChanged {
        time: utc_now(),
        file_id: ctx.accounts.file.id,
        authority: ctx.accounts.file.authority,
    });
    Ok(())
}
