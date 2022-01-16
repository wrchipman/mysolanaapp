use anchor_lang::prelude::*;

declare_id!("9Mbjs3YuRtKMmhpoAzHfvb2GxySrTLpdTqoV33GuWihH");

#[program]
mod mysolanaapp {
    use super::*;
    
    pub fn create(ctx: Context<Create>) -> ProgramResult {
       let base_account = &mut ctx.accounts.base_account;
       base_account.count = 0;
       Ok(()) 
    }
    pub fn increment(ctx: Context<Increment>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count += 1;
        Ok(())
    }
}

//Transaction instructions
#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 16 +16)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>, 
}

//Transaction Instructions
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

//An Account that goes inside a transaction instruction
#[account]
pub struct BaseAccount {
    pub count: u64,
}