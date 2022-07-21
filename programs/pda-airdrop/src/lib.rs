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

    pub fn execute_airdrop(ctx: Context<ExecuteAirdrop>, amount: u64) -> Result<()> {
        // Find bump for mint account created in create_airdrop
        // let (_, bump) = Pubkey::find_program_address(
        //     &[], 
        //     ctx.program_id
        // );
        let bump  = *ctx.bumps.get("mint").unwrap();

        // mint amount to associated token account
        let token_program = ctx.accounts.token_program.to_account_info();
        let mint_to_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.ata.to_account_info(),
            authority: ctx.accounts.mint.to_account_info(),
        };
        mint_to(
            CpiContext::new_with_signer(
                token_program, 
                mint_to_accounts, 
                &[&[
                    &[],
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
pub struct CreateAirdrop<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init, 
        payer = signer, 
        seeds = [b"mint".as_ref()],
        bump,
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
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(init_if_needed, payer = signer, associated_token::mint = mint, associated_token::authority = signer)]
    pub ata: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
