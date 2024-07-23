use crate::utils::utc_now;
use crate::{state::file::*, utils::validate_string_len};
use anchor_lang::prelude::*;

// SPACE SIZE:
// + 8 discriminator
// + 32 id (Pubkey)
// + 32 authority (Pubkey)
// + 1 + 32 Option<backup> (Pubkey)
// + 4 + 16 name (string)
// + 8 roles_updated_at
// + 8 rules_updated_at
// + 1 cached
// + 1 + 8 Option<u64> fee
// + 1 account_type
// + 1 + 8 Option<i64> expires_at
// + 1 bump
// total = 8 + 32  + 32 + 1 + 32 + 4 + 16 + 8 + 8 + 1 + 1 + 8 + 1 + 1 + 8 + 1 = 162
#[derive(Accounts)]
#[instruction(file_data: FileData)]
pub struct InitializeFiles<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 162,
        seeds = [b"file".as_ref(), file_data.id.key().as_ref()], 
        bump
    )]
    pub file: Box<Account<'info, File>>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_files(ctx: Context<InitializeFiles>, file_data: FileData) -> Result<()> {
    let file = &mut ctx.accounts.file;
    file.id = file_data.id;
    file.account_type = AccountTypes::Basic as u8;
    file.authority = ctx.accounts.authority.key();
    file.recovery = file_data.recovery;
    file.name = validate_string_len(&file_data.name, 0, 16)?;
    file.fee = None;
    file.cached = file_data.cached;
    file.rules_updated_at = utc_now();
    file.roles_updated_at = file.rules_updated_at;
    file.expires_at = None;
    file.bump = ctx.bumps.file;
    emit!(FileChanged {
        time: file.rules_updated_at,
        file_id: ctx.accounts.file.id,
        authority: ctx.accounts.file.authority,
    });
    Ok(())
}
