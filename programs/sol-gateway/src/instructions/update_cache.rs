use crate::state::file::*;
use crate::utils::utc_now;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateCache<'info> {
    pub authority: Signer<'info>, // Only current Authority is allowed
    #[account(
        mut,
        has_one = authority,
        seeds = [b"file".as_ref(), file.id.key().as_ref()], 
        bump = file.bump,
    )]
    pub file: Account<'info, File>,
    pub system_program: Program<'info, System>,
}

pub fn update_cache(ctx: Context<UpdateCache>, cache_updated: u8) -> Result<()> {
    let file = &mut ctx.accounts.file;
    let now = utc_now();
    if cache_updated == CacheUpdated::Roles as u8 {
        file.roles_updated_at = now;
    } else {
        file.rules_updated_at = now;
    }
    emit!(FileChanged {
        time: now,
        file_id: ctx.accounts.file.id,
        authority: ctx.accounts.file.authority,
    });
    Ok(())
}
