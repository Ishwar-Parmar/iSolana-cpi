use anchor_lang::prelude::*;
use solana_program::program::invoke_signed;
use add_num::cpi::accounts::Adder;
use add_num::program::AddNumbers;
use add_num::{self, Total};


declare_id!("ABoYG2GWbzLgnnGhK2pUGNupzKoYe7UGk2idrAXbstAS");


#[program]
pub mod i_cpi {
    use super::*;
    // Get the account info of the target program
    pub fn summation(ctx: Context<Summation>,  num1: u32, num2: u32,) -> Result<()> {
        msg!("number 1: {}", &num1);
        msg!("number 2: {}", &num2);

        let target_program_info = ctx.accounts.add_num.clone();

        // Create the accounts required by the instruction
        let account_one = ctx.accounts.adder.clone();
        seeds = &[
            b"add_two_num",
            &target_program_info.key.to_bytes(),
        ];

        // Derive the PDA address
        let (pda, _) = Pubkey::find_program_address(seeds, ctx.program_id);

        // Define the accounts required for the instruction
        let accounts = vec![account_one.clone(), AccountMeta::new(pda, false)];

        // Specify the target program's program ID
        let target_program_id = target_program_info.key;

        let accounts =  vec![account_one.clone(), pda.clone()];

        // Construct the instruction
        let add_instruction = anchor_lang::solana_program::instruction::Instruction {
            program_id: *target_program_id,
            accounts,
        };

        invoke_signed(
            &add_instruction,                   // instruction to invoke
            pda.key(),                    // seeds to derive pda
           
            &[
                ctx.accounts.add_pda.to_account_info(), // this is only possible to sign here, if we get this account from ctx.accounts
            ],
        )?;
        Ok(())
    }
}


#[derive(Accounts)]
pub struct Summation<'info> {
    #[account(mut)]
    pub adder: Account<'info, Adder>,
    #[account(mut)]
    // Since AccountInfo therefore not kept seeds here in constraints // seeds = [b"GlobalTreasury".as_ref(), game_list_account.key().as_ref()], bump
    pub add_pda: AccountInfo<'info>, 
    pub add_num: Program<'info, AddNumbers>,
}
