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
        fund.donations = 0;
        Ok(())
    }

    pub fn donate(ctx: Context<Donate>, amount: u64) -> ProgramResult {
        let fund = &mut ctx.accounts.fund;
        fund.donations += 1;
        let donation = &mut ctx.accounts.donation;
        donation.bump = *ctx.bumps.get("donation").unwrap();
        donation.donor = ctx.accounts.donor.key();
        donation.amount = amount;

        let rent = donation.to_account_info().lamports();

        if amount < rent {
            return Err(ProgramError::InvalidArgument)
        }

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.donor.key(),
            &ctx.accounts.fund.key(),
            amount - rent,
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

    pub fn close(ctx: Context<Close>) -> ProgramResult {
        if ctx.accounts.fund.founder != ctx.accounts.founder.key()
        {
            return Err(ProgramError::IllegalOwner)
        }
        let fund = &mut ctx.accounts.fund;
        if fund.donations == 0
        {
            return Err(ProgramError::InvalidAccountData)
        }
        fund.donations -= 1;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> ProgramResult {
        if ctx.accounts.fund.founder != ctx.accounts.founder.key()
        {
            return Err(ProgramError::IllegalOwner)
        }
        if ctx.accounts.fund.donations != 0 {
            return Err(ProgramError::InvalidAccountData)
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(
        init,
        rent_exempt = enforce,
        payer = founder,
        space = Fund::LEN,
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
    #[account(
        init,
        rent_exempt = enforce,
        payer = donor,
        space = Donation::LEN,
        seeds = [b"donation", &fund.donations.to_be_bytes()],
        bump
    )]
    pub donation: Account<'info, Donation>,
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

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    /// CHECK: this one must be the founder stored in fund
    pub founder: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"donation", &(fund.donations - 1).to_be_bytes()],
        bump = donation.bump,
        close = founder)]
    pub donation: Account<'info, Donation>,
    #[account(mut)]
    pub fund: Account<'info, Fund>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Fund {
    pub bump: u8,
    pub founder: Pubkey,
    pub donations: u64,
}

impl Fund {
    const LEN: usize = 8 + 1 + 32 + 8;
}

#[account]
pub struct Donation {
    pub bump: u8,
    pub donor: Pubkey,
    pub amount: u64,
}

impl Donation {
    const LEN: usize = 8 + 1 + 32 + 8;
}
