use anchor_spl::{metadata::MetadataAccount, token::TokenAccount};
use crate::instructions::allowed::{allowed, AllowedRule};
use crate::state::file::{File, Seed};
use crate::state::role::Role;
use crate::state::rule::*;
use crate::utils::utc_now;
use anchor_lang::prelude::*;
use crate::metadata_program;


#[derive(Accounts)]
pub struct DeleteRule<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        close = collector,
        seeds = [rule.namespace.to_le_bytes().as_ref(), rule.role.as_ref(), rule.resource.as_ref(), rule.permission.as_ref(), sol_gateway_file.id.key().as_ref()], 
        bump = rule.bump,
    )]
    pub rule: Account<'info, Rule>,
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
    #[account(
        seeds = [sol_gateway_rule2.namespace.to_le_bytes().as_ref(), sol_gateway_rule2.role.as_ref(), sol_gateway_rule2.resource.as_ref(), sol_gateway_rule2.permission.as_ref(), sol_gateway_rule2.file_id.key().as_ref()],
        bump = sol_gateway_rule2.bump,
    )]
    pub sol_gateway_rule2: Option<Box<Account<'info, Rule>>>,
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

pub fn delete_rule(
    ctx: Context<DeleteRule>
) -> Result<()> {
      // Checks if is allowed to delete a rule for this specific Namespace and Role.
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
            namespace: Namespaces::DeleteRuleNSRole as u8,
            resource: ctx.accounts.rule.namespace.to_string(),
            roles: ctx.accounts.rule.roles.clone(),
        },
    )?;
    // // Checks if is allowed to delete a rule for this specific Resource and Permission.
    allowed(
        &ctx.accounts.signer,
        &ctx.accounts.sol_gateway_file,
        &ctx.accounts.sol_gateway_role,
        &ctx.accounts.sol_gateway_rule2,
        &ctx.accounts.sol_gateway_token,
        &ctx.accounts.sol_gateway_metadata,
        &mut None,
        &ctx.accounts.system_program,
        AllowedRule {
            file_id: ctx.accounts.sol_gateway_file.id.key(),
            namespace: Namespaces::DeleteRuleResourcePerm as u8,
            resource: ctx.accounts.rule.resource.to_string(),
            roles: ctx.accounts.rule.roles.clone(),
        },
    )?;

    emit!(RulesChanged {
        time: utc_now(),
        file_id: ctx.accounts.sol_gateway_file.id,
    });
    Ok(())
}
