use anchor_lang::prelude::*;

declare_id!("7WEsVmbRMFMJ5TXBuauiQRA7BLU7sgGj6ytZLdShvVme");

#[program]
pub mod sample {
    use super::*;


    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
