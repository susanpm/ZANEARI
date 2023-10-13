use anchor_lang::prelude::*;

declare_id!("AZ52kixdoCyRwrkAWirWYpBrfpi7wbSGxxf6TNyxVys8");

#[program]
pub mod zaneari {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
