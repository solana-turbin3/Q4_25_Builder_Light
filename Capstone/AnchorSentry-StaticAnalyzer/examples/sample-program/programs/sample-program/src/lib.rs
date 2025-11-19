// programs/capstone_escrow/src/lib.rs
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface , TransferChecked, CloseAccount, close_account},
};

declare_id!("GWKBYXfT5EMPa6avVVk3J7xTZJi3AY6cNQtQXHRfJaXZ");

#[account]
pub struct Escrow {
    pub seed: u64,
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive: u64,
    pub bump: u8,
}

#[program]
pub mod sample_program {
    use super::*;

    // 1. Initialize escrow
    pub fn init_escrow(ctx: Context<Make>, seed: u64, receive: u64) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        escrow.seed = seed;
        escrow.maker = ctx.accounts.maker.key();
        escrow.mint_a = ctx.accounts.mint_a.key();
        escrow.mint_b = ctx.accounts.mint_b.key();
        escrow.receive = receive;
        escrow.bump = ctx.bumps.escrow;

        Ok(())
    }

    // 2. Maker deposits token A into vault
    pub fn deposit(ctx: Context<Deposit>, deposit: u64) -> Result<()> {
        let transfer_accounts = TransferChecked {
            from: ctx.accounts.maker_ata_a.to_account_info(),
            mint: ctx.accounts.mint_a.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.maker.to_account_info(),
        };
        
        let _scale = 100_000 / deposit;

        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer_accounts);
        transfer_checked(cpi_ctx, deposit, ctx.accounts.mint_a.decimals)
    }

    // 3. Taker deposits token B → maker receives it
    pub fn deposit_taker(ctx: Context<DepositTaker>) -> Result<()> {
        let transfer_accounts = TransferChecked {
            from: ctx.accounts.taker_ata_b.to_account_info(),
            mint: ctx.accounts.mint_b.to_account_info(),
            to: ctx.accounts.maker_ata_b.to_account_info(),
            authority: ctx.accounts.taker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer_accounts);
        transfer_checked(cpi_ctx, ctx.accounts.escrow.receive, ctx.accounts.mint_b.decimals)
    }

    // 4. Taker withdraws token A + close vault
    pub fn withdraw_and_close_vault(ctx: Context<WithdrawAndClose>) -> Result<()> {
        let escrow = &ctx.accounts.escrow;

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"escrow",
            escrow.maker.as_ref(),
            &escrow.seed.to_le_bytes(),
            &[escrow.bump],
        ]];

        // Transfer A from vault → taker
        let transfer_accounts = TransferChecked {
            from: ctx.accounts.vault.to_account_info(),
            mint: ctx.accounts.mint_a.to_account_info(),
            to: ctx.accounts.taker_ata_a.to_account_info(),
            authority: ctx.accounts.escrow.to_account_info(),
        };

        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_accounts,
            signer_seeds,
        );
        transfer_checked(transfer_ctx, ctx.accounts.vault.amount, ctx.accounts.mint_a.decimals)?;

        // Close vault and refund rent to maker
        let close_accounts = CloseAccount {
            account: ctx.accounts.vault.to_account_info(),
            destination: ctx.accounts.maker.to_account_info(),
            authority: ctx.accounts.escrow.to_account_info(),
        };

        let close_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            close_accounts,
            signer_seeds,
        );
        close_account(close_ctx)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
//  Context structs
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Accounts)]
#[instruction(seed: u64, bump: u8)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    
    #[account(mut)]
    /// CHECK: Verified??
    pub authority: AccountInfo<'info>,

    #[account(mint::token_program = token_program)]
    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        space = 8 + 8 + 16 + 16,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = authority,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(mut, has_one = mint_a)]
    pub escrow: Account<'info, Escrow>,

    #[account(mut)]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct DepositTaker<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    /// CHECK: Verified via escrow constraints
    pub maker: UncheckedAccount<'info>,

    #[account(mint::token_program = token_program)]
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,

    #[account(has_one = mint_b, has_one = maker)]
    pub escrow: Account<'info, Escrow>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawAndClose<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    /// CHECK: Verified via escrow constraints
    pub maker: UncheckedAccount<'info>,

    #[account(mint::token_program = token_program)]
    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [b"escrow", maker.key().as_ref(), &escrow.seed.to_le_bytes()],
        bump = escrow.bump,
        has_one = maker,
        has_one = mint_a,
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(mut)]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}