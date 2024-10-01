use crate::utils::utc_now;
use crate::{state::file::*, utils::validate_string_len};
use crate::{Errors, FileMetadata};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(file_data: FileData)]
pub struct InitializeFiles<'info> {
    #[account(mut)]
    pub contributor: Signer<'info>,

    #[account(
        init_if_needed,
        payer = rent_payer,
        space = File::MAX_SIZE,
        seeds = [b"file".as_ref(), file_data.id.key().as_ref()], 
        bump
    )]
    pub file: Box<Account<'info, File>>,

    #[account(
        init_if_needed,
        payer = rent_payer,
        space = FileMetadata::MAX_SIZE,
        seeds = [b"metadata".as_ref(), file_data.id.key().as_ref()],
        bump,
    )]
    pub file_metadata: Option<Account<'info, FileMetadata>>,

    #[account(mut)]
    pub rent_payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_files(ctx: Context<InitializeFiles>, file_data: FileData) -> Result<()> {
    let file = &mut ctx.accounts.file;
    file.id = file_data.id;
    file.authority = ctx.accounts.contributor.key();
    file.recovery = file_data.recovery;
    file.fid = validate_string_len(&file_data.fid, 0, 32)?; //
    file.fee = None;
    file.size = file_data.size;
    file.checksum = validate_string_len(&file_data.checksum, 0, 32)?;
    file.rules_updated_at = utc_now();
    file.roles_updated_at = file.rules_updated_at;
    file.expires_at = file_data.expires_at;
    file.bump = ctx.bumps.file;

    // Initialize metadata if provided
    match (file_data.metadata, &mut ctx.accounts.file_metadata) {
        (Some(metadata), Some(file_metadata)) => {
            file_metadata.file_id = file.id;
            file_metadata.metadata = metadata;
            file_metadata.bump = ctx.bumps.file_metadata;
        }
        (Some(_), None) => {
            return err!(Errors::FileMetadataAccountNotFound);
        }
        (None, Some(_)) => {
            return err!(Errors::UnexpectedMetadataAccount);
        }
        (None, None) => {
            // No metadata provided and no metadata account
        }
    }

    emit!(FileChanged {
        time: file.rules_updated_at,
        file_id: ctx.accounts.file.id,
        authority: ctx.accounts.file.authority,
    });
    Ok(())
}
