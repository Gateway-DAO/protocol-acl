use crate::state::file::File;
use crate::state::role::{Role, RolesChanged};
use crate::utils::utc_now;
use crate::Errors;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct DeleteAssignedRole<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        close = collector,
        seeds = [role.address.as_ref(), file.id.key().as_ref()],
        bump = role.bump,
    )]
    pub role: Account<'info, Role>,

    #[account(
        seeds = [b"file".as_ref(), file.id.key().as_ref()],
        bump = file.bump,
    )]
    pub file: Box<Account<'info, File>>,

    #[account(
        seeds = [signer.key().as_ref(), file.id.key().as_ref()],
        bump = user_role.bump,
        constraint = user_role.address == signer.key(),
    )]
    pub user_role: Box<Account<'info, Role>>,

    /// CHECK: This account collects lamports from the closed account
    #[account(mut)]
    pub collector: AccountInfo<'info>,
}

pub fn delete_assigned_role(ctx: Context<DeleteAssignedRole>) -> Result<()> {
    let user_role = &ctx.accounts.user_role;
    let role_to_delete = &ctx.accounts.role;
    let file = &ctx.accounts.file;

    // Check if the signer is the authority of the file
    if ctx.accounts.signer.key() != file.authority {
        if !user_role.can_share {
            return Err(Errors::InsufficientSharePermission.into());
        }
        if user_role.permission_level < role_to_delete.permission_level {
            return Err(Errors::CannotDeleteHigherPermissionLevel.into());
        }
    }

    emit!(RolesChanged {
        time: utc_now(),
        file_id: ctx.accounts.file.id,
    });
    Ok(())
}
