use crate::instructions::allowed::{allowed, AllowedRule};
use crate::metadata_program;
use crate::state::file::{File, Seed};
use crate::state::role::*;
use crate::state::rule::{Namespaces, Rule};
use crate::utils::{roles::address_or_wildcard, rules::*, utc_now};
use crate::Errors::InvalidRole;
use anchor_lang::prelude::*;
use anchor_spl::{metadata::MetadataAccount, token::TokenAccount};

// SPACE SIZE:
// + 8 discriminator
// + 32 file_id (Pubkey)
// + 1 + 32 address Option<Pubkey>
// + 4 + 16 role (string)
// + 1 + 1 address_type
// + 1 + 8 expires_at Option<i64>
// + 1 bump
// total = 8 + 32 + 1 + 32 + 4 + 16 + 1 + 1 +  1 + 8 + 1 = 105
#[derive(Accounts)]
#[instruction(assign_role_data:AssignPermissionData)]
pub struct AssignRole<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 105,
        seeds = [assign_role_data.role.as_ref(), address_or_wildcard(&assign_role_data.address), sol_gateway_file.id.key().as_ref()],
        constraint = valid_rule(&assign_role_data.role, true)  @ InvalidRole,
        bump
    )]
    pub role: Account<'info, Permission>,
    #[account(
        seeds = [b"file".as_ref(), sol_gateway_file.id.key().as_ref()],
        bump = sol_gateway_file.bump,
    )]
    pub sol_gateway_file: Box<Account<'info, File>>,
    #[account(
        seeds = [sol_gateway_role.role.as_ref(),  address_or_wildcard(&sol_gateway_role.address), sol_gateway_role.file_id.key().as_ref()],
        bump = sol_gateway_role.bump
    )]
    pub sol_gateway_role: Option<Box<Account<'info, Permission>>>,
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
        payer = signer,
        space = 9, // Account discriminator + initialized
        seeds = [b"seed".as_ref(), signer.key.as_ref()],
        bump
    )]
    pub sol_gateway_seed: Option<Account<'info, Seed>>,
    pub system_program: Program<'info, System>,
}

pub fn assign_role(ctx: Context<AssignRole>, assign_role_data: AssignPermissionData) -> Result<()> {
    allowed(
        &ctx.accounts.signer,
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
            permission: assign_role_data.role.clone(),
        },
    )?;

    let role = &mut ctx.accounts.role;
    role.bump = ctx.bumps.role;
    role.file_id = ctx.accounts.sol_gateway_file.id;
    role.address = assign_role_data.address;
    role.role = assign_role_data.role;
    role.address_type = assign_role_data.address_type;
    role.expires_at = assign_role_data.expires_at;
    emit!(PermissionsChanged {
        time: utc_now(),
        file_id: ctx.accounts.sol_gateway_file.id,
    });
    Ok(())
}
