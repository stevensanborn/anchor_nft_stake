use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct StakeConfig{

    pub points_per_stake: u8, //points will be given for stake
    pub max_stake: u8, // 
    pub freeze_period: u32, // could be an enum but his is a u32
    pub reward_period: u8, // ?
    pub rewards_bump:u8,
    pub bump: u8,

}


