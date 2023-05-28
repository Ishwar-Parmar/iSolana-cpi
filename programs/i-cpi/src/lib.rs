use anchor_lang::prelude::*;
use add_num::cpi::accounts::Adder;
use add_num::program::AddNumbers;
use add_num::{self, Total};


declare_id!("ABoYG2GWbzLgnnGhK2pUGNupzKoYe7UGk2idrAXbstAS");


#[program]
pub mod i_cpi {
    use super::*;
    pub fn summation(ctx: Context<Summation>,  num1: u32, num2: u32,) -> Result<()> {
        msg!("number 1: {}", &n1);
        msg!("number 2: {}", &n2);

        add_num::cpi::adder(
            CpiContext::new(
                ctx.accounts.add_num.to_account_info(),

                let cpi_accounts = AddNumbers {
                    total: ctx.accounts.adder.to_account_info(),
                };
            ),
            num1, num2
        )
        Ok(())
    }
}


#[derive(Accounts)]
pub struct Summation<'info> {
    #[account(mut)]
    pub adder: Account<'info, Adder>,
    pub add_num: Program<'info, AddNumbers>,
}
