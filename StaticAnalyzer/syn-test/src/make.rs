use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

#[program]
pub mod anchor_escrow_q4_25 {
    pub fn init_escrow(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        ctx.accounts.deposit(deposit)?;
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)
    }

}

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct OpenEscrowAccount<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[accounts(
        seeds = [b"pool".as_ref(), mint.key.as_ref()], bump
    )]
    pub pool: Account<'info>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[accounts(
        init,
        payer=admin,
        space=[8+8+16+32+(16*2)],
        associated_token::mint = mint,
        associated_token::authority=pool
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>
}

