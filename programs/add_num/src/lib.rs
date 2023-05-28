use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod add_num {
    use super::*;

    pub fn add_numbers(
        ctx: Context<AddNumbers>,
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
pub struct AddNumbers<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub sum: Account<'info, Total>,
}

#[account]
pub struct Total {
    pub total: u64,
}
