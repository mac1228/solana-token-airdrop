use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

declare_id!("AUrpX9QjAFeKBSBC2acgYHzBUud6xeiR2VvmiJSdoHqk");

#[program]
pub mod pda_airdrop {
    use super::*;

    pub fn create_airdop(_ctx: Context<CreateAirdrop>) -> Result<()> {
        Ok(())
    }

    pub fn execute_airdrop(_ctx: Context<ExecuteAirdrop>, _amount: u64) -> Result<()> {
        Ok(())
    }
    // pub fn execute_airdrop(ctx: Context<ExecuteAirdrop>, amount: u64) -> Result<()> {
    //     // mint amount to associated token account
    //     let token_program = ctx.accounts.token_program.to_account_info();
    //     let mint_to_accounts = MintTo {
    //         mint: ctx.accounts.mint.to_account_info(),
    //         to: ctx.accounts.ata.to_account_info(),
    //         authority: ctx.accounts.mint.to_account_info(),
    //     };
    //     let bump = *ctx.bumps.get("mint").unwrap();
    //     mint_to(
    //         CpiContext::new_with_signer(
    //             token_program, 
    //             mint_to_accounts, 
    //             &[&[
    //                 b"mint",
    //                 &[bump]
    //             ]]
    //         ), 
    //         amount
    //     )?;

    //     Ok(())
    // }
}

const DISCRIMINATOR: usize = 8;
const U8: usize = 1;
const PUBLIC_KEY: usize = 32;

// Account: Airdrop
#[account]
pub struct Airdrop {
    bump: u8,
    mint: Pubkey
}

impl Airdrop {
    const LEN: usize = DISCRIMINATOR + U8 + PUBLIC_KEY;
}

// Instruction: Create Mint Account for Airdrop
#[derive(Accounts)]
pub struct CreateAirdrop<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init, 
        seeds = [b"airdrop".as_ref()], 
        bump, 
        payer = signer, 
        space = Airdrop::LEN
    )]
    pub airdrop: Account<'info, Airdrop>,
    #[account(
        init, 
        seeds = [b"mint".as_ref()], 
        bump,
        payer = signer,
        mint::decimals = 0, 
        mint::authority = airdrop
    )]
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

// Instruction: Execute Airdrop
#[derive(Accounts)]
pub struct ExecuteAirdrop<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [b"airdrop".as_ref()], bump)]
    pub airdrop: Account<'info, Airdrop>,
    #[account(mut, seeds = [b"mint".as_ref()], bump)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
