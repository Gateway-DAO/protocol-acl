use anchor_lang::prelude::*;
pub use constants::*;
use errors::*;
use instructions::*;
pub use sol_gateway_macros;
use state::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

declare_id!("C8TANLzc5UKGQBzhmKjrs7nAB326zxoBFtJ9x48C5S6Z");

#[program]
pub mod sol_gateway {

    use super::*;

    pub fn initialize_files(ctx: Context<InitializeFiles>, file_data: FileData) -> Result<()> {
        instructions::initialize_files::initialize_files(ctx, file_data)
    }

    pub fn update_file(ctx: Context<UpdateFile>, file_data: UpdateFileData) -> Result<()> {
        instructions::update_file::update_file(ctx, file_data)
    }

    pub fn delete_file(ctx: Context<DeleteFile>) -> Result<()> {
        instructions::delete_file::delete_file(ctx)
    }

    pub fn update_file_metadata(
        ctx: Context<UpdateFileMetadata>,
        metadata_data: MetadataData,
    ) -> Result<()> {
        instructions::update_metadata::update_file_metadata(ctx, metadata_data)
    }

    pub fn add_rule(ctx: Context<AddRule>, rule_data: RuleData) -> Result<()> {
        instructions::add_rule::add_rule(ctx, rule_data)
    }

    pub fn delete_rule(ctx: Context<DeleteRule>) -> Result<()> {
        instructions::delete_rule::delete_rule(ctx)
    }

    pub fn assign_role(ctx: Context<AssignRole>, assign_role_data: AssignRoleData) -> Result<()> {
        instructions::assign_role::assign_role(ctx, assign_role_data)
    }

    pub fn delete_assigned_role(ctx: Context<DeleteAssignedRole>) -> Result<()> {
        instructions::delete_assigned_role::delete_assigned_role(ctx)
    }

    /**
     * Updates either file.roles_updated_at or file.rules_updated_at fields, so clients
     * can keep track and cache roles & rules accordingly.
     */
    pub fn update_cache(ctx: Context<UpdateCache>, cache_updated: u8) -> Result<()> {
        instructions::update_cache::update_cache(ctx, cache_updated)
    }

    /**
     * Checks if the current user is authorized to run the instruction,
     * throwing "Unauthorized" error otherwise.
     */
    pub fn allowed(ctx: Context<Allowed>, allowed_rule: AllowedRule) -> Result<()> {
        instructions::allowed::allowed(
            &ctx.accounts.signer,
            &ctx.accounts.sol_gateway_file,
            &ctx.accounts.sol_gateway_role,
            &ctx.accounts.sol_gateway_rule,
            &ctx.accounts.sol_gateway_token,
            &ctx.accounts.sol_gateway_metadata,
            &mut ctx.accounts.sol_gateway_seed,
            &ctx.accounts.system_program,
            allowed_rule,
        )
    }
}
