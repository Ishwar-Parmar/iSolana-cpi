use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, program::invoke_signed, system_instruction},
};


declare_id!("EnjN3cm7xYqYHNUZbQfhJYj5S5RBrSU9tc5aHwQ6LqvT");


#[program]
pub mod i_cpi {
    use super::*;
    pub fn summation(ctx: Context<Summation>, n1: u64, n2: u64) -> Result<()> {
        
        let total = &mut ctx.accounts.sum;

        msg!("number 1: {}", &n1);
        msg!("number 2: {}", &n2);

        total.total = &n1 + &n2;

        Ok(())
    }
}


#[derive(Accounts)]
pub struct Summation<'info> {
    #[account(init, payer = payer, space = 8 + 8)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


