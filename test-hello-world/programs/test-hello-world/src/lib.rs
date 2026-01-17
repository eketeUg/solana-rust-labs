use anchor_lang::prelude::*;

declare_id!("6z4YkFzXsAKrqrnHPfiqnnZpwAg2wnJr5A2nEFEoXV6G");

#[program]
pub mod test_hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
