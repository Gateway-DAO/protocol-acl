use crate::state::file::{File, FileChanged};
use crate::utils::file::allowed_authority;
use crate::utils::utc_now;
use crate::Errors;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct DeleteFile<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        close = collector,
        constraint = allowed_authority(&authority.key(), &file.authority)  @ Errors::Unauthorized,
        seeds = [b"file".as_ref(), file.id.key().as_ref()], 
        bump = file.bump,
    )]
    pub file: Account<'info, File>,
    /// CHECK: collector of the funds
    #[account(mut)]
    collector: AccountInfo<'info>,
}

pub fn delete_file(ctx: Context<DeleteFile>) -> Result<()> {
    emit!(FileChanged {
        time: utc_now(),
        file_id: ctx.accounts.file.id,
        authority: ctx.accounts.file.authority,
    });
    Ok(())
}
