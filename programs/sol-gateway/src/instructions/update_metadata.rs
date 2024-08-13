use crate::{
    state::file::*, utils::allowed_authority, Errors, FileMetadata, Metadata, MetadataUpdated,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateFileMetadata<'info> {
    pub signer: Signer<'info>,
    #[account(
        seeds = [b"file".as_ref(), file.id.key().as_ref()],
        bump = file.bump,
        constraint = file.authority == signer.key() || (file.recovery.is_some() && file.recovery.unwrap() == signer.key()) @ Errors::UnauthorizedMetadataUpdate,
    )]
    pub file: Box<Account<'info, File>>,
    #[account(
        mut,
        seeds = [b"metadata".as_ref(), file.id.key().as_ref()],
        bump = file_metadata.bump,
    )]
    pub file_metadata: Account<'info, FileMetadata>,
    pub system_program: Program<'info, System>,
}

pub fn update_file_metadata(
    ctx: Context<UpdateFileMetadata>,
    file_id: Pubkey,
    new_metadata: Vec<Metadata>,
) -> Result<()> {
    let file = &ctx.accounts.file;
    let file_metadata = &mut ctx.accounts.file_metadata;

    require!(
        allowed_authority(&ctx.accounts.signer.key(), &file.authority),
        Errors::UnauthorizedMetadataUpdate
    );

    file_metadata.metadata = new_metadata;

    emit!(MetadataUpdated {
        time: Clock::get()?.unix_timestamp,
        file_id,
        authority: ctx.accounts.file.authority,
    });

    Ok(())
}
