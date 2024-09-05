use crate::instructions::allowed::{allowed, AllowedRule};
use crate::metadata_program;
use crate::state::file::{File, Seed};
use crate::state::role::*;
use crate::state::rule::{Namespaces, Rule};
use crate::utils::{rules::*, utc_now};
use crate::Errors::InvalidRole;
use anchor_lang::prelude::*;
use anchor_spl::{metadata::MetadataAccount, token::TokenAccount};

#[derive(Accounts)]
#[instruction(assign_role_data:AssignRoleData)]
pub struct AssignRole<'info> {
    #[account(mut)]
    pub contributor: Signer<'info>,

    #[account(
        init,
        payer = rent_payer,
        space = 105,
        seeds = [assign_role_data.address.as_ref(), sol_gateway_file.id.key().as_ref()],
        constraint = valid_rule(&assign_role_data.roles, true) @ InvalidRole,
        bump
    )]
    pub role: Account<'info, Role>,

    /** Validation accounts */
    #[account(
        seeds = [b"file".as_ref(), sol_gateway_file.id.key().as_ref()],
        bump = sol_gateway_file.bump,
    )]
    pub sol_gateway_file: Box<Account<'info, File>>,
    #[account(
        seeds = [sol_gateway_role.address.as_ref(), sol_gateway_role.file_id.key().as_ref()],
        bump = sol_gateway_role.bump
    )]
    pub sol_gateway_role: Option<Box<Account<'info, Role>>>,
    #[account(
        seeds = [sol_gateway_rule.namespace.to_le_bytes().as_ref(), sol_gateway_rule.role.as_ref(), sol_gateway_rule.resource.as_ref(), sol_gateway_rule.permission.as_ref(), sol_gateway_rule.file_id.key().as_ref()],
        bump = sol_gateway_rule.bump,
    )]
    pub sol_gateway_rule: Option<Box<Account<'info, Rule>>>,
    #[account()]
    pub sol_gateway_token: Option<Box<Account<'info, TokenAccount>>>,
    #[account(
        seeds = [b"metadata", metadata_program::ID.as_ref(), sol_gateway_metadata.mint.key().as_ref()],
        seeds::program = metadata_program::ID,
        bump,
    )]
    pub sol_gateway_metadata: Option<Box<Account<'info, MetadataAccount>>>,
    #[account(
        init_if_needed,
        payer = rent_payer,
        space = 9, // Account discriminator + initialized
        seeds = [b"seed".as_ref(), contributor.key.as_ref()],
        bump
    )]
    pub sol_gateway_seed: Option<Account<'info, Seed>>,

    #[account(mut)]
    pub rent_payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn assign_role(ctx: Context<AssignRole>, assign_role_data: AssignRoleData) -> Result<()> {
    allowed(
        &ctx.accounts.contributor,
        &ctx.accounts.sol_gateway_file,
        &ctx.accounts.sol_gateway_role,
        &ctx.accounts.sol_gateway_rule,
        &ctx.accounts.sol_gateway_token,
        &ctx.accounts.sol_gateway_metadata,
        &mut ctx.accounts.sol_gateway_seed,
        &ctx.accounts.system_program,
        AllowedRule {
            file_id: ctx.accounts.sol_gateway_file.id.key(),
            namespace: Namespaces::AssignRole as u8,
            resource: assign_role_data.address_type.to_string(),
            roles: assign_role_data.roles.clone(),
        },
    )?;

    let role = &mut ctx.accounts.role;
    role.bump = ctx.bumps.role;
    role.file_id = ctx.accounts.sol_gateway_file.id;
    role.address = assign_role_data.address;
    role.roles = assign_role_data.roles;
    role.address_type = assign_role_data.address_type;
    role.expires_at = assign_role_data.expires_at;

    emit!(RolesChanged {
        time: utc_now(),
        file_id: ctx.accounts.sol_gateway_file.id,
    });
    Ok(())
}
