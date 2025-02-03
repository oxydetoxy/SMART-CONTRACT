use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};
use anchor_spl::associated_token::AssociatedToken;
// In program



declare_id!("6tqbc3RuNZGnNBZS7eo6DgdYzfbeXHdajob1B4WeLmXJ");
//const DEPLOYER_PUBKEY: Pubkey = pubkey!("8LeYKLjf73Lkju1r9dcdAzvtFLZA1wcwpVLJQ5iMaYyW");

#[program]
pub mod testing {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let stake_pool = &mut ctx.accounts.stake_pool;
        stake_pool.authority = ctx.accounts.authority.key();
        stake_pool.total_staked = 0;
        Ok(())
    }

    pub fn stake_nft(ctx: Context<StakeNft>) -> Result<()> {
        let stake_pool = &mut ctx.accounts.stake_pool;
        let user_stake_info = &mut ctx.accounts.user_stake_info;

        // Transfer NFT to program's custody
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.user_nft_account.to_account_info(),
                    to: ctx.accounts.program_nft_account.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                }
            ),
            1
        )?;

        // Update stake information
        user_stake_info.user = ctx.accounts.user.key();
        user_stake_info.nft_mint = ctx.accounts.nft_mint.key();
        user_stake_info.stake_start_time = Clock::get()?.unix_timestamp;
        user_stake_info.is_staked = true;

        stake_pool.total_staked += 1;

        msg!("NFT staked successfully!");
        Ok(())
    }

    pub fn unstake_nft(ctx: Context<UnstakeNft>) -> Result<()> {
        let stake_pool = &mut ctx.accounts.stake_pool;
        let user_stake_info = &mut ctx.accounts.user_stake_info;

        // Validate NFT is actually staked
        require!(user_stake_info.is_staked, ErrorCode::NotStaked);
        
        // Prepare PDA signer for token transfer
        let mint_key = ctx.accounts.nft_mint.key();
    
    // Prepare PDA signer for token transfer
    let seeds = &[
        b"nft_auth",
        mint_key.as_ref(),
        &[ctx.bumps.nft_auth],
    ];
    let signer = [&seeds[..]];

        // Transfer NFT back to user
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.program_nft_account.to_account_info(),
                    to: ctx.accounts.user_nft_account.to_account_info(),
                    authority: ctx.accounts.nft_auth.to_account_info(),
                },
                &signer
            ),
            1
        )?;

        // Update stake status
        user_stake_info.is_staked = false;
        stake_pool.total_staked -= 1;

        msg!("NFT unstaked successfully!");
        Ok(())
    }
}

// Account Structures

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8,
        seeds = [b"stake_pool"],
        bump
       
    )]
    pub stake_pool: Account<'info, StakePool>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct StakeNft<'info> {
    #[account(mut, seeds = [b"stake_pool"], bump)]
    pub stake_pool: Account<'info, StakePool>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 32 + 32 + 8 + 1,
        seeds = [b"user_stake_info", user.key().as_ref(), nft_mint.key().as_ref()],
        bump
    )]
    pub user_stake_info: Account<'info, UserStakeInfo>,

    #[account(
        seeds = [b"nft_auth", nft_mint.key().as_ref()],
        bump
    )]
    /// CHECK: PDA used as token authority
    pub nft_auth: UncheckedAccount<'info>,

    pub nft_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = user_nft_account.mint == nft_mint.key(),
        constraint = user_nft_account.owner == user.key(),
        constraint = user_nft_account.amount == 1
    )]
    pub user_nft_account: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = user,
        seeds = [b"nft_account", nft_mint.key().as_ref()],
        bump,
        token::mint = nft_mint,
        token::authority = nft_auth
    )]
    pub program_nft_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct UnstakeNft<'info> {
    #[account(mut, seeds = [b"stake_pool"], bump)]
    pub stake_pool: Account<'info, StakePool>,

    #[account(
        mut,
        seeds = [b"user_stake_info", user.key().as_ref(), nft_mint.key().as_ref()],
        bump
    )]
    pub user_stake_info: Account<'info, UserStakeInfo>,

    #[account(
        seeds = [b"nft_auth", nft_mint.key().as_ref()],
        bump
    )]
    /// CHECK: PDA used as token authority
    pub nft_auth: UncheckedAccount<'info>,

    pub nft_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = user_nft_account.mint == nft_mint.key(),
        constraint = user_nft_account.owner == user.key()
    )]
    pub user_nft_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"nft_account", nft_mint.key().as_ref()],
        bump,
        token::mint = nft_mint,
        token::authority = nft_auth
    )]
    pub program_nft_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

// Data Structures

#[account]
pub struct StakePool {
    pub authority: Pubkey,
    pub total_staked: u64,
}

#[account]
pub struct UserStakeInfo {
    pub user: Pubkey,
    pub nft_mint: Pubkey,
    pub stake_start_time: i64,
    pub is_staked: bool,
}

// Error Codes
#[error_code]
pub enum ErrorCode {
    #[msg("NFT is not currently staked")]
    NotStaked,
}
