use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct StakeAccount{

        pub owner :Pubkey,
        pub mint:Pubkey,
        pub staked_at:i64, //when it was staked 
        pub bump:u8,

}



