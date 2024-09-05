use crate::instructions::allowed::{allowed, AllowedRule};
use crate::metadata_program;
use crate::state::file::{File, Seed};
use crate::state::role::{Role, RolesChanged};
use crate::state::rule::Namespaces;
use crate::state::rule::Rule;
use crate::utils::utc_now;
use anchor_lang::prelude::*;
use anchor_spl::{metadata::MetadataAccount, token::TokenAccount};

#[derive(Accounts)]
pub struct DeleteAssignedRole<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        close = collector,
        seeds = [role.address.as_ref(), sol_gateway_file.id.key().as_ref()],
        bump = role.bump,
    )]
    pub role: Account<'info, Role>,
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
        seeds::program =metadata_program::ID,
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
    /// CHECK: collector of the funds
    #[account(mut)]
    collector: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn delete_assigned_role(ctx: Context<DeleteAssignedRole>) -> Result<()> {
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
            namespace: Namespaces::DeleteAssignRole as u8,
            resource: ctx.accounts.role.address_type.to_string(),
            roles: ctx.accounts.role.roles.clone(),
        },
    )?;

    emit!(RolesChanged {
        time: utc_now(),
        file_id: ctx.accounts.sol_gateway_file.id,
    });
    Ok(())
}
