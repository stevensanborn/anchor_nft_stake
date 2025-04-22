



use std::ops::Rem;

use anchor_lang::prelude::*;

use anchor_spl::token::{Mint,Token,TokenAccount};

use anchor_spl::{metadata::{mpl_token_metadata::instructions::{FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts,ThawDelegatedAccountCpi ,ThawDelegatedAccountCpiAccounts}, MasterEditionAccount, Metadata, MetadataAccount}, token::{approve, Approve, Revoke,revoke}};
use crate::state::{StakeAccount, StakeConfig,UserAccount};
use crate::errors::StakeError::{FreezePeriodNotOver};
#[derive(Accounts)]

pub struct Stake<'info>{

    #[account(mut)]
    pub user:Signer<'info>,

    pub mint: Account<'info,Mint>,
    pub collection_mint: Account<'info,Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub mint_ata :Account<'info,TokenAccount>,

    #[account(
        seeds = [b"metadata",
        metadata_program.key().as_ref(),
        mint.key().as_ref(),
        ],
        seeds::program= metadata_program.key(),
        constraint=metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint=metadata.collection.as_ref().unwrap().verified,
        bump
    )]
    pub metadata: Account<'info,MetadataAccount>,


    #[account(
        seeds = [b"metadata",
        metadata_program.key().as_ref(),
        mint.key().as_ref(),
        b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump
    )]
    pub master_edition:Account<'info,MasterEditionAccount>,


    #[account(
        init,
        payer = user,
        seeds = [b"stake",config.key().as_ref(),mint.key().as_ref()],
        bump,
        space = 8 + StakeAccount::INIT_SPACE,
    )]
    pub stake_account:Account<'info,StakeAccount>,

    #[account
    (
        mut,
        seeds = [b"config"],
        bump = config.bump,
    )]
    pub config:Account<'info,StakeConfig>,

    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump,

    )]
    pub user_account:Account<'info,UserAccount>,
    pub metadata_program: Program<'info, Metadata>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}


impl<'info> Stake<'info> {
   
   pub fn stake(&mut self, bumps: &StakeBumps )->Result<()>{

    assert!(self.user_account.amount_staked <= self.config.max_stake, "You have reached the maximum stake limit");
    
    let cpi_program = self.token_program.to_account_info();

    let cpi_accounts  = Approve {
        to: self.mint_ata.to_account_info(),
        delegate: self.stake_account.to_account_info(),
        authority: self.user.to_account_info(),
    };
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    approve(cpi_context, 1)?;


    let delegate = &self.stake_account.to_account_info();
    let token_account  = &self.mint_ata.to_account_info();
    let edition  = &self.master_edition.to_account_info();
    let mint = &self.mint.to_account_info();
    let token_program = &self.token_program.to_account_info();

    let metadata = &self.metadata.to_account_info();
    
    let seeds = &[
        b"stake",
        self.config.to_account_info().key.as_ref(),
        self.mint.to_account_info().key.as_ref(),
        &[bumps.stake_account],
    ];
    let signer = &[&seeds[..]];
    let freeze_delegated_account_cpi = FreezeDelegatedAccountCpiAccounts { 
        
        delegate,
        token_account,
        edition,
        mint,
        token_program,
    };
    FreezeDelegatedAccountCpi::new(
        metadata,
        freeze_delegated_account_cpi
    ).invoke_signed(signer)?;
    
    Ok(())
    
   }



   pub fn thaw(&mut self,bumps:&StakeBumps) -> Result<()>{
    
    let cpi_program = self.token_program.to_account_info();

    //verif y the thawing period throw freeze error
    assert!(
        (self.stake_account.staked_at + self.config.freeze_period as i64) < Clock::get().unwrap().unix_timestamp,
        "Freeze period is not over"
    );

    //thaw the token account


    let delegate = &self.stake_account.to_account_info();
    let token_account  = &self.mint_ata.to_account_info();
    let edition  = &self.master_edition.to_account_info();
    let mint = &self.mint.to_account_info();
    let token_program = &self.token_program.to_account_info();
    let metadata = &self.metadata.to_account_info();

    let thaw_delegated_account_cpi = ThawDelegatedAccountCpiAccounts { 
        
        delegate,
        token_account,
        edition,
        mint,
        token_program,
    };

    
    let seeds = &[
        b"stake",
        self.config.to_account_info().key.as_ref(),
        self.mint.to_account_info().key.as_ref(),
        &[bumps.stake_account],
    ];
    let signer = &[&seeds[..]];

    ThawDelegatedAccountCpi::new(
        metadata,
        thaw_delegated_account_cpi
    ).invoke_signed(signer)?;

    //revoke the token account
    let cpi_accounts = Revoke {
        source: self.mint_ata.to_account_info(),
        authority: self.stake_account.to_account_info(),
    };

    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    revoke(cpi_context)?;

    
    
    Ok(())
   }
}