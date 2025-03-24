use anchor_lang::prelude::*;

declare_id!("EMDedCPiESfktXgEfF5FKgtzTCsPQNiB5A737PB4Nuh9");

#[program]
mod counter {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter=&mut ctx.accounts.counter;
        msg!("Counter account created!.Current count: {}", counter.count); // Message will show up in the tx logs
        Ok(())
    }
    pub fn increment(ctx:Context<Increment>)->Result<()>{
        let counter=&mut ctx.accounts.counter;
        msg!("Previous counter:{}",counter.count);
        counter.count=counter.count.checked_add(1).unwrap();
        msg!("Counter incremented! Current count:{}",counter.count);
        Ok(())
    }
    pub fn decrement(ctx:Context<Decrement>)->Result<()>{
        let counter=&mut ctx.accounts.counter;
        msg!("Previous counter:{}",counter.count);
        counter.count=counter.count.checked_sub(1).unwrap();
        msg!("Counter decremented! Current count:{}",counter.count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // We must specify the space in order to initialize an account.
    // First 8 bytes are default account discriminator,
    // next 8 bytes come from NewAccount.data being type u64.
    // (u64 = 64 bits unsigned integer = 8 bytes)
    #[account(mut)]
    pub signer:Signer<'info>,
    #[account(init, payer = signer, space = 8 + 8)]
    pub counter:Account<'info ,Counter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info>{
    #[account(mut)]
    pub counter:Account<'info,Counter>,
}

#[derive(Accounts)]
pub struct Decrement<'info>{
    #[account[mut]]
    pub counter:Account<'info,Counter>,
}

#[account]
pub struct Counter {
    count:u64,
}