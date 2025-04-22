use anchor_lang::prelude::*;
pub mod state;
pub mod instructions;
pub mod errors;
pub use instructions::initialize_config::*;
pub use instructions::initialize_user::*;
use crate::instructions::stake::*;

declare_id!("5Db35Czu6kZ5rQiPt7nwo9hiQ7bHBS8Pt5HTTjrnzVVF");

#[program]
pub mod stake {
    use super::*;

    pub fn initialize(ctx: Context<InitializeConfig>,points_per_stake:u8, max_stake:u8 ,freeze_period:u32 ) -> Result<()> {
        
       
       ctx.accounts.initialize_config(
            points_per_stake, //points_per_stake
            max_stake, //max_stake
            freeze_period, // freeze_period
            &ctx.bumps // bumps
        )
        
    }

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.initialize_user(&ctx.bumps)
    }


    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)
    }
    // pub fn unstake(ctx: Context<Stake>, amount:u64) -> Result<()> {
    //     // ctx.accounts.thaw();
    //     // ctx.accounts.revoke()
    //     Ok(())
    // }


}

