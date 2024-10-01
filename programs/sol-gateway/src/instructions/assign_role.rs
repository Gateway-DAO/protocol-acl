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
        constraint = file.authority == contributor.key() || (user_role.is_some() && user_role.as_ref().unwrap().can_share),
    )]
    pub file: Box<Account<'info, File>>,

    pub user_role: Option<Account<'info, Role>>,

    #[account(mut)]
    pub rent_payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn assign_role(ctx: Context<AssignRole>, assign_role_data: AssignRoleData) -> Result<()> {
    let contributor_key = ctx.accounts.contributor.key();
    let file = &ctx.accounts.file;

    // Verify if role is between 1 and 3
    if assign_role_data.permission_level < 1 || assign_role_data.permission_level > 3 {
        return Err(Errors::InvalidPermissionLevel.into());
    }

    if contributor_key != file.authority {
        let user_role = match &ctx.accounts.user_role {
            Some(ur) => ur,
            None => return Err(Errors::MissingUserRole.into()),
        };

        // Verify that the user_role PDA matches the expected address
        let (expected_user_role_key, _) = Pubkey::find_program_address(
            &[contributor_key.as_ref(), file.id.key().as_ref()],
            ctx.program_id,
        );
        if user_role.key() != expected_user_role_key {
            return Err(Errors::InvalidUserRole.into());
        }

        if !user_role.can_share {
            return Err(Errors::InsufficientSharePermission.into());
        }
        if assign_role_data.permission_level > user_role.permission_level {
            return Err(Errors::CannotGrantHigherPermissionLevel.into());
        }
        if assign_role_data.can_share && !user_role.can_share {
            return Err(Errors::CannotGrantSharePermission.into());
        }
    }

    // Initialize or update the Role account
    let role = &mut ctx.accounts.role;
    role.bump = ctx.bumps.role;
    role.file_id = file.id;
    role.address = assign_role_data.address;
    role.permission_level = assign_role_data.permission_level;
    role.can_share = assign_role_data.can_share;
    role.address_type = assign_role_data.address_type;
    role.expires_at = assign_role_data.expires_at;

    emit!(RolesChanged {
        time: utc_now(),
        file_id: file.id,
    });
    Ok(())
}
