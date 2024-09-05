use crate::state::file::{File, FileChanged};
use crate::state::role::{Role, RoleType};
use crate::utils::file::allowed_authority;
use crate::utils::roles::allowed_roles;
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
        constraint = allowed_authority(&authority.key(), &file.authority) || allowed_roles(&role.as_ref().unwrap().roles, &vec![RoleType::Delete]) @ Errors::Unauthorized,
        seeds = [b"file".as_ref(), file.id.key().as_ref()], 
        bump = file.bump,
    )]
    pub file: Account<'info, File>,

    #[account(
        seeds = [authority.key().as_ref(), file.id.key().as_ref()],
        bump = role.bump,
    )]
    pub role: Option<Account<'info, Role>>,

    #[account(mut)]
    /// CHECK: collector is an account that doesn't need to sign/be checked
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
