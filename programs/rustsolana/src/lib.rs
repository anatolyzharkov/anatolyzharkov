use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod rustsolana {

    use anchor_lang::solana_program::entrypoint::ProgramResult;
    use super::*;
    pub fn create(ctx: Context<Create>) -> ProgramResult {
        let fund = &mut ctx.accounts.fund;
        fund.founder = ctx.accounts.founder.key();
        fund.bump = *ctx.bumps.get("fund").unwrap();
        Ok(())
    }

    pub fn donate(ctx: Context<Donate>, amount: u64) -> ProgramResult {
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.donor.key(),
            &ctx.accounts.fund.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.donor.to_account_info(),
                ctx.accounts.fund.to_account_info(),
            ],
        )?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> ProgramResult {
        if ctx.accounts.fund.founder != ctx.accounts.founder.key()
        {
            Err(ProgramError::IllegalOwner)
        }
        else {
            Ok(())
        }
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(
        init,
        payer = founder,
        space = 8 + 1 + 32,
        seeds = [b"fund"],
        bump
    )]
    pub fund: Account<'info, Fund>,
    #[account(mut)]
    pub founder: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)]
    pub donor: Signer<'info>,
    #[account(mut, seeds = [b"fund"], bump = fund.bump)]
    pub fund: Account<'info, Fund>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    /// CHECK: this one must be the founder stored in fund
    pub founder: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"fund"], bump = fund.bump, close = founder)]
    pub fund: Account<'info, Fund>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Fund {
    pub bump: u8,
    pub founder: Pubkey,
}
