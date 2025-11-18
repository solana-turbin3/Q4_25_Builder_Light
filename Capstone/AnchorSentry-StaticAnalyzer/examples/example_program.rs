// use anchor_lang::prelude::*;
// use anchor_spl::{
//     associated_token::AssociatedToken,
//     token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
// };

// #[account]
// pub struct VaultAccount {
//     pub authority: Pubkey,   // Creator of the game session
//     pub session_id: u64,    // Required bet amount per player
//     pub created_at: i64,     // Creation timestamp
//     // pub name: String,
//     pub bump: u8,            // PDA bump
//     pub vault_bump: u8,      // Add this field for vault PDA bump
// }

// #[program]
// pub mod anchor_escrow_q4_25 {
//     pub fn init_escrow(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
//         ctx.accounts.deposit(deposit)?;
//         ctx.accounts.init_escrow(seed, receive, &ctx.bumps);

//         let fee = deposit / receive * 100;

//         let slm = (deposit / deposit) * 100;
//     }

//     pub fn init_second_escrow(ctx: Context<OpenEscrowAccount>, deposit: u64, receive: u64) -> Result<()> {
//         ctx.accounts.deposit(deposit)?;
//         ctx.accounts.init_escrow(seed, receive, &ctx.bumps)
//     }

// }

// #[derive(Accounts)]
// #[instruction(seed: u64)]
// pub struct OpenEscrowAccount<'info> {
//     #[account(mut)]
//     pub maker: Signer<'info>,
//     #[accounts(
//         seeds = [b"pool".as_ref(), mint.key.as_ref()], bump
//     )]
//     pub pool: Account<'info>,
//     #[account(
//         mint::token_program = token_program
//     )]
//     pub mint: InterfaceAccount<'info, Mint>,
//     #[accounts(
//         init,
//         payer=admin,
//         associated_token::mint = mint,
//         associated_token::authority=pool
//     )]
//     pub token_account: Account<'info, TokenAccount>,
//     pub associated_token_program: Program<'info, AssociatedToken>
// }

// #[derive(Accounts)]
// pub struct InitializeVault<'info> {
//     #[account(
//         init,
//         // seeds = [b"vault"],
//         bump,
//         payer = payer,
//         space = 8 + 32 + 8 + 8 + 1 + 1 + 2,
//     )]
//     pub vault: Account<'info, VaultAccount>,
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }
pub fn main (){
    
}