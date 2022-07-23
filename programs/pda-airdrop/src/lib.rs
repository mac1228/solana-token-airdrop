use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

declare_id!("AUrpX9QjAFeKBSBC2acgYHzBUud6xeiR2VvmiJSdoHqk");

#[program]
pub mod pda_airdrop {
    use super::*;

    pub fn create_airdrop_mint(_ctx: Context<CreateAirdropMint>) -> Result<()> {
        Ok(())
    }

    pub fn execute_airdrop(ctx: Context<ExecuteAirdrop>, amount: u64) -> Result<()> {
        // mint amount to associated token account
        let token_program = ctx.accounts.token_program.to_account_info();
        let mint_to_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.ata.to_account_info(),
            authority: ctx.accounts.mint.to_account_info(),
        };
        let bump = *ctx.bumps.get("mint").unwrap();
        mint_to(
            CpiContext::new_with_signer(
                token_program, 
                mint_to_accounts, 
                &[&[
                    b"mint",
                    &[bump]
                ]]
            ), 
            amount
        )?;

        Ok(())
    }
}

// Instruction: Create Mint Account for Airdrop
#[derive(Accounts)]
pub struct CreateAirdropMint<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init, 
        seeds = [b"mint".as_ref()], 
        bump,
        payer = signer,
        mint::decimals = 0, 
        mint::authority = mint
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
    #[account(mut, seeds = [b"mint".as_ref()], bump)]
    pub mint: Account<'info, Mint>,
    #[account(init, payer = signer, associated_token::mint = mint, associated_token::authority = signer)]
    pub ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
