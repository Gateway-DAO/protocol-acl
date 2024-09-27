use crate::state::file::File;
use crate::state::role::*;
use crate::utils::utc_now;
use crate::Errors;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(assign_role_data: AssignRoleData)]
pub struct AssignRole<'info> {
    #[account(mut)]
    pub contributor: Signer<'info>,

    #[account(
        init_if_needed,
        payer = rent_payer,
        space = 105,
        seeds = [assign_role_data.address.as_ref(), file.id.key().as_ref()],
        bump,
    )]
    pub role: Account<'info, Role>,

    #[account(
        seeds = [b"file".as_ref(), file.id.key().as_ref()],
        bump = file.bump,
    )]
    pub file: Box<Account<'info, File>>,

    #[account(
        seeds = [contributor.key().as_ref(), file.id.key().as_ref()],
        bump = user_role.bump,
        constraint = user_role.address == contributor.key(),
    )]
    pub user_role: Box<Account<'info, Role>>,

    #[account(mut)]
    pub rent_payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn assign_role(ctx: Context<AssignRole>, assign_role_data: AssignRoleData) -> Result<()> {
    let user_role = &ctx.accounts.user_role;

    // Validate that the assigning user has the Share permission
    if !user_role.can_share {
        return Err(Errors::InsufficientSharePermission.into());
    }

    // Validate that the permission level being assigned is less than or equal to the assigning user's level
    if assign_role_data.permission_level > user_role.permission_level {
        return Err(Errors::CannotGrantHigherPermissionLevel.into());
    }

    // Validate that the can_share flag is only true if the assigning user has can_share
    if assign_role_data.can_share && !user_role.can_share {
        return Err(Errors::CannotGrantSharePermission.into());
    }

    // Initialize or update the Role account
    let role = &mut ctx.accounts.role;
    role.bump = ctx.bumps.role;
    role.file_id = ctx.accounts.file.id;
    role.address = assign_role_data.address;
    role.permission_level = assign_role_data.permission_level;
    role.can_share = assign_role_data.can_share;
    role.address_type = assign_role_data.address_type;
    role.expires_at = assign_role_data.expires_at;

    emit!(RolesChanged {
        time: utc_now(),
        file_id: ctx.accounts.file.id,
    });
    Ok(())
}
