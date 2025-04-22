

use anchor_lang::prelude::*;


#[error_code]
pub enum  StakeError{

    #[msg("The max stake has been reached")]
    MaxStakeReached,
    #[msg("The freeze period is not over")] 
    FreezePeriodNotOver,

}