use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod add_num {
    use super::*;
    pub fn initialize(_ctx: Context<InitializeAdd>) -> Result<()> {
        Ok(())
    }

    pub fn adder(
        ctx: Context<Adder>,
        num1: u32,
        num2: u32,
    ) -> Result<()> {
        let sum = num1 + num2;
        let total = &mut ctx.accounts.sum;
        total.total = sum;
        msg!("The sum of {} and {} is {}", num1, num2, sum);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeAdd<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub power: Account<'info, Total>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Adder<'info> {
    #[account(mut)]
    pub sum: Account<'info, Total>,
}

#[account]
pub struct Total {
    pub total: u64,
}
