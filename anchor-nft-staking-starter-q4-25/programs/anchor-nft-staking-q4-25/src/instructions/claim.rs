use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

use crate::{
    errors::StakeError,
    state::{StakeConfig, UserAccount},
};

#[derive(Accounts)]
pub struct Claim<'info> {
    //TODO
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = reward_mint,
        associated_token::authority = reward_mint,
        associated_token::token_program = token_program
    )]
    pub rewards_ata: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"config".as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, StakeConfig>,

    #[account(
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        mut,
        seeds = [b"rewards".as_ref(), config.key().as_ref()],
        bump,
    )]
    pub reward_mint: Account<'info, Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Claim<'info> {
    //TODO
    pub fn claim(&mut self) -> Result<()> {
        let amount = self.user_account.points as u64;

        require!(amount > 0, StakeError::ZeroClaim);

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"config",
            &[self.config.bump],
        ]];

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.reward_mint.to_account_info(),
                to: self.rewards_ata.to_account_info(),
                authority: self.config.to_account_info(),
            },
            signer_seeds,
        );

        mint_to(cpi_ctx, amount)?;

        self.user_account.points = 0;

        Ok(())
    }
}