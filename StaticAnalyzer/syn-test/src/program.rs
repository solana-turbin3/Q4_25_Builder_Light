use anchor_lang::prelude::*;

// A simple escrow that supports depositing and claiming $GOR tokens in a game
declare_id!("FWkvTDXiXaaFbFSKVzewnjzb52rryC5h9atifJuZepU8");

#[account]
pub struct VaultAccount {}

#[program]
pub mod game_escrow {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        Ok(())
    }

    pub fn bet(ctx: Context<Bet>) -> Result<()> {
        anchor_lang::solana_program::program::invoke(
            &anchor_lang::solana_program::system_instruction::transfer(
                &ctx.accounts.player.key(),
                &ctx.accounts.vault.key(),
                100_000_000,
            ),
            &[
                ctx.accounts.player.to_account_info(),
                ctx.accounts.vault.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
        Ok(())
    }

    pub fn claim_win(ctx: Context<ClaimWin>) -> Result<()> {
        let amount = 200_000_000;
        **ctx
            .accounts
            .vault
            .to_account_info()
            .try_borrow_mut_lamports()? -= amount;
        **ctx
            .accounts
            .player
            .to_account_info()
            .try_borrow_mut_lamports()? += amount;
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Bet<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'info, VaultAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimWin<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'info, VaultAccount>,
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
