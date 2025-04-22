
use anchor_lang::prelude::*;

use anchor_spl::token::{Mint,Token,TokenAccount};
use crate::state::{StakeConfig,StakeAccount};

#[derive(Accounts)]

pub struct InitializeConfig<'info>{

    #[account(mut)]
    pub admin:Signer<'info>, 
    
    #[account(
        init,
        payer = admin,
        space = 8 + StakeAccount::INIT_SPACE,
        seeds = [b"config".as_ref()],
        bump
    )]
    pub config: Account<'info,StakeConfig>,

    //generate the rewards mint
    #[account(
        init,
        payer = admin,
        seeds = [b"rewards".as_ref(), config.key().as_ref()],
        mint::decimals = 6,
        mint::authority = config,
        bump,
    )]
    rewards_mint:Account<'info,Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}


impl<'info> InitializeConfig<'info> {
   
    pub fn initialize_config(&mut self, points_per_stake:u8, max_stake:u8 , freeze_period:u32, bumps:&InitializeConfigBumps )->Result<()>{

    self.config.set_inner(StakeConfig{
        points_per_stake,
        max_stake,
        freeze_period, 
        reward_period: 0, //? not usign this yet 
        rewards_bump: bumps.rewards_mint,
        bump: bumps.config,
    });
    
    Ok(())
   }
}