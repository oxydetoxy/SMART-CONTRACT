use anchor_lang::prelude::*;

declare_id!("AQurAZkuP6T5ZYB63seEECKByk9mXLG6jqaXToBhhcU4");

#[program]
pub mod nft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
