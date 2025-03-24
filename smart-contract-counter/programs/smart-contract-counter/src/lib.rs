use anchor_lang::prelude::*;

declare_id!("9AMX3iy96Pu2Y7wuHg6aF8dTQDBFvnZmnhpqTFg4DxmC");

#[program]
mod counter {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter=&mut ctx.accounts.counter;
        counter.bump=ctx.bumps.counter;//store bump in Counter account
        msg!("Counter account created!.Current count: {}", counter.count);
        msg!("Counter bump:{}",counter.bump); // Message will show up in the tx logs
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
    pub user:Signer<'info>,

    //Create and initialize 'Counter' account using PDA as the address
    #[account(init,
              seeds=[b"counter"],
              bump,
              payer = user, 
              space = 8 + Counter::INIT_SPACE
            )]
    pub counter:Account<'info ,Counter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info>{
    #[account(mut,
              seeds=[b"counter"],
              bump=counter.bump,
            )]
    pub counter:Account<'info,Counter>,
}

#[derive(Accounts)]
pub struct Decrement<'info>{
    #[account(mut,
             seeds=[b"counter"]
             bump=counter.bump
            )]
    pub counter:Account<'info,Counter>,
}

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count:u64,//8 bytes
    pub bump:u8//1 byte
}