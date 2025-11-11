use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
// A simple escrow that supports depositing and claiming $GOR tokens in a game
declare_id!("FWkvTDXiXaaFbFSKVzewnjzb52rryC5h9atifJuZepU8");

#[account]
pub struct VaultAccount {}

#[account]
pub struct BetAccount {}

#[program]
pub mod game_escrow {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(
        init,
        seeds = [b"vault"],
        bump,
        payer = payer,
        space = 8,
    )]
    pub vault: Account<'info, VaultAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

