use crate::{
    role::Role,
    state::file::*,
    utils::{allowed_authority, perform_action},
    Errors, FileMetadata, MetadataData, MetadataUpdated,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateFileMetadata<'info> {
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"file".as_ref(), file.id.key().as_ref()], 
        bump = file.bump,
        constraint = file.authority == authority.key() || (file.recovery.is_some() && file.recovery.unwrap() == authority.key()) || (role.is_some() && perform_action(&role.as_ref().unwrap().permission_level, crate::ActionType::Update)) @ Errors::InsufficientPermission,
    )]
    pub file: Box<Account<'info, File>>,

    #[account(
        mut,
        seeds = [b"metadata".as_ref(), file.id.key().as_ref()],
        bump = file_metadata.bump,
    )]
    pub file_metadata: Account<'info, FileMetadata>,

    #[account(
        seeds = [authority.key().as_ref(), file.id.key().as_ref()],
        bump = role.bump,
    )]
    pub role: Option<Box<Account<'info, Role>>>,

    pub system_program: Program<'info, System>,
}

pub fn update_file_metadata(
    ctx: Context<UpdateFileMetadata>,
    metadata_data: MetadataData,
) -> Result<()> {
    let file = &ctx.accounts.file;
    let file_metadata = &mut ctx.accounts.file_metadata;

    require!(
        allowed_authority(&ctx.accounts.authority.key(), &file.authority),
        Errors::UnauthorizedMetadataUpdate
    );

    file_metadata.metadata = metadata_data.metadata;

    emit!(MetadataUpdated {
        time: Clock::get()?.unix_timestamp,
        file_id: file.id,
        authority: ctx.accounts.file.authority,
    });

    Ok(())
}
