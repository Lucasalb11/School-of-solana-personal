//-------------------------------------------------------------------------------
///
/// TASK: Implement the withdraw functionality for the on-chain vault
/// 
/// Requirements:
/// - Verify that the vault is not locked
/// - Verify that the vault has enough balance to withdraw
/// - Transfer lamports from vault to vault authority
/// - Emit a withdraw event after successful transfer
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::errors::VaultError;
use crate::events::WithdrawEvent;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    /// Vault authority must sign
    #[account(mut)]
    pub vault_authority: Signer<'info>,

    /// Vault account, holds lamports
    #[account(
        mut,
        seeds = [b"vault", vault.vault_authority.as_ref()],
        bump,
        constraint = vault.vault_authority == vault_authority.key() @ ProgramError::IllegalOwner
    )]
    pub vault: Account<'info, Vault>,
}

pub fn _withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let vault = &ctx.accounts.vault;

    // Vault must not be locked
    require!(!vault.locked, VaultError::VaultLocked);

    // Check balance and rent-exemption
    let vault_info = vault.to_account_info();
    let current = vault_info.lamports();
    let rent = Rent::get()?;
    let min_balance = rent.minimum_balance(Vault::INIT_SPACE);

    // Must leave enough for rent-exemption after withdrawal
    require!(
        current >= amount && current - amount >= min_balance,
        VaultError::InsufficientBalance
    );

    // Transfer lamports from vault PDA to authority by modifying lamports directly
    **vault_info.try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.vault_authority.to_account_info().try_borrow_mut_lamports()? += amount;

    emit!(WithdrawEvent {
        amount,
        vault_authority: vault.vault_authority,
        vault: vault.key(),
    });

    Ok(())
}