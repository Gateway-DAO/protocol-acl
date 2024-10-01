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

    /// The signer's role account (optional if signer is the file authority)
    pub user_role: Option<Account<'info, Role>>,

    /// CHECK: This account collects lamports from the closed account
    #[account(mut)]
    pub collector: AccountInfo<'info>,
}

pub fn delete_assigned_role(ctx: Context<DeleteAssignedRole>) -> Result<()> {
    let signer_key = ctx.accounts.signer.key();
    let file = &ctx.accounts.file;
    let role_to_delete = &ctx.accounts.role;

    if signer_key != file.authority {
        let user_role = match &ctx.accounts.user_role {
            Some(ur) => ur,
            None => return Err(Errors::MissingUserRole.into()),
        };

        // Verify that the user_role PDA matches the expected address
        let (expected_user_role_key, _) = Pubkey::find_program_address(
            &[signer_key.as_ref(), file.id.key().as_ref()],
            ctx.program_id,
        );
        if user_role.key() != expected_user_role_key {
            return Err(Errors::InvalidUserRole.into());
        }

        // Perform validations
        if !user_role.can_share {
            return Err(Errors::InsufficientSharePermission.into());
        }
        if user_role.permission_level < role_to_delete.permission_level {
            return Err(Errors::CannotDeleteHigherPermissionLevel.into());
        }
    }

    emit!(RolesChanged {
        time: utc_now(),
        file_id: file.id,
    });
    Ok(())
}
