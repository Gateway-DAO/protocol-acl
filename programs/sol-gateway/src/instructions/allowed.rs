use anchor_spl::{metadata::MetadataAccount, token::TokenAccount};
use crate::state::{File, Seed};
use crate::state::rule::Rule;
use crate::utils::{allowed_perm, utc_now, allowed_authority, get_fee, subtract_rent_exemption_from_fee};
use crate::state::role::Role;
use crate::metadata_program;
use anchor_lang::prelude::*;
use crate::Errors::{Unauthorized, InvalidFileID, MissingSeedAccount};

#[derive(Accounts)]
pub struct Allowed<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        seeds = [b"file".as_ref(), sol_gateway_file.id.key().as_ref()], 
        bump = sol_gateway_file.bump,
    )]
    pub sol_gateway_file: Box<Account<'info, File>>,
    #[account(
        seeds = [sol_gateway_rule.namespace.to_le_bytes().as_ref(), sol_gateway_rule.permission_level.to_le_bytes().as_ref(), sol_gateway_rule.resource.as_ref(), sol_gateway_rule.permission.as_ref(), sol_gateway_rule.file_id.key().as_ref()], 
        bump = sol_gateway_rule.bump,
    )]
    pub sol_gateway_rule: Option< Box<Account<'info, Rule>>>,
    #[account(
        seeds = [sol_gateway_role.address.as_ref(), sol_gateway_role.file_id.key().as_ref()], 
        bump = sol_gateway_role.bump
    )]
    pub sol_gateway_role: Option< Box<Account<'info, Role>>>,
    #[account()]
    pub sol_gateway_token: Option< Box<Account<'info, TokenAccount>>>,
    #[account(
        seeds = [b"metadata", metadata_program::ID.as_ref(), sol_gateway_metadata.mint.key().as_ref()],
        seeds::program = metadata_program::ID,
        bump,
    )]
    pub sol_gateway_metadata: Option< Box<Account<'info, MetadataAccount>>>,
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

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct AllowedRule {
    pub file_id: Pubkey,
    pub namespace: u8,
    pub resource: String,
    pub permission_level: u8
}


pub fn allowed<'info>(
    signer: &Signer<'info>,
    file: &Box<Account<'info, File>>,
    role: &Option<Box<Account<'info, Role>>>,
    rule: &Option<Box<Account<'info, Rule>>>,
    token: &Option<Box<Account<'info, TokenAccount>>>,
    metadata: &Option<Box<Account<'info, MetadataAccount>>>,
    seed: &mut Option<Account<'info, Seed>>,
    system_program: &Program<'info, anchor_lang::system_program::System>,
    allowed_rule: AllowedRule) -> Result<()> {
    // The FILE ID must be the one authorized by the program
    if allowed_rule.file_id != file.id.key(){
        return Err(error!(InvalidFileID))
    }
    
    // FILE Authority is always allowed (No fees)
    if allowed_authority(&signer.key(), &file.authority.key()){
        return Ok(());
    }

    let mut fee:  u64 = get_fee(file);
    // Seed account is mandatory when Fee is defined and using normal "Rule"
    if fee > 0 && seed.is_none() {
        return Err(error!(MissingSeedAccount))
    }

    // Initialize Seed account (if needed)
    // First call to "allowed()" on each wallet initializes a seed account which will be used to collect fees
    // therefore the rent exemption fee from the account should be deducted from the regular fee.
    if seed.is_some() && !seed.as_ref().unwrap().initialized{
        fee = subtract_rent_exemption_from_fee(fee);
        seed.as_mut().map(|s| {
            s.initialized = true;
        });
    }

    // Rule or Role can only be empty when using Authority
    if rule.is_none() || role.is_none(){
        return Err(error!(Unauthorized))
    }

    let rule = rule.as_ref().unwrap();
    let role = role.as_ref().unwrap();

    // The FILE ID must match on: FILE, Role, Rule
    if file.id != rule.file_id  || file.id != role.file_id{
        return Err(error!(Unauthorized))
    }

    // Check Rule is within the corresponding Namespace
    if rule.namespace != allowed_rule.namespace  {
        return Err(error!(Unauthorized))
    }

    // Check Resource & Permission
    if !allowed_perm(&allowed_rule.resource, &rule.resource) || rule.permission_level < allowed_rule.permission_level {
        return Err(error!(Unauthorized))
    }

    let now = utc_now();
    // Check if role expired
    if role.expires_at.is_some() && role.expires_at.unwrap() <= now{
        return Err(error!(Unauthorized))
    }
    // Check if rule expired 
    if rule.expires_at.is_some() && rule.expires_at.unwrap() <= now{
        return Err(error!(Unauthorized))
    }
    // Check if the wallet is authorized (Address = "None" is considered wildcard "*")
      if signer.key() == role.address {
            return pay_fee(system_program, signer, seed, fee);
    }
    // Check if the file or Collection Mint addresses are authorized
    if token.is_some(){
        let token = token.as_ref().unwrap();
        // Check if is the real owner of the file and has at least one
        if token.owner != signer.key() || token.amount <= 0{
            return Err(error!(Unauthorized))
        }
        // File authorized (Address = "None" is considered wildcard "*")
        if token.mint == role.address {
            return pay_fee(system_program, signer, seed, fee);
        }
        if  metadata.is_some() {
            let metadata = metadata.as_ref().unwrap();
            if metadata.collection.is_some() && metadata.mint == token.mint {
                let collection = metadata.collection.as_ref().unwrap();
                // Collection authorized (Address = "None" is considered wildcard "*")
                if collection.verified && collection.key == role.address{
                    return pay_fee(system_program, signer, seed, fee);
                }
            }
        }
    }

    Err(error!(Unauthorized))
}

/// Pay fee (when defined)
pub fn pay_fee<'info>(system_program:&Program<'info, anchor_lang::system_program::System>, payer:&Signer<'info>, receiver:&Option<Account<'info,Seed>>, fee:u64)-> Result<()>{
    if fee > 0 {
        if receiver.is_none(){
            return Err(error!(MissingSeedAccount));
        }
        let cpi_context = CpiContext::new(
            system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: payer.to_account_info(),
                to: receiver.as_ref().unwrap().to_account_info(),
            },
        );
        anchor_lang::system_program::transfer(cpi_context, fee)?;
    }

    Ok(())
}
