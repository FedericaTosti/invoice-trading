use anchor_lang::prelude::*;

use crate::state::{Borrower, Investor, Investment};

#[derive(Accounts)]
pub struct CreateBorrower<'info> {
    #[account(init, payer = user, space = Borrower::LEN)]
    pub borrower: Account<'info, Borrower>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateInvestor<'info> {
    #[account(init, payer = user, space = Investor::LEN)]
    pub investor: Account<'info, Investor>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateInvestment<'info> {
    #[account(init, payer = user, space = Investment::LEN)]
    pub investment: Account<'info, Investment>,
    #[account(mut)]
    pub borrower: Account<'info, Borrower>,
    #[account(mut)]
    pub investor: Account<'info, Investor>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FundInvoice<'info> {
    #[account(mut)]
    pub invoice: Account<'info, Invoice>,
    #[account(mut)]
    pub investor: Account<'info, Investor>,
    #[account(mut)]
    pub investor_wallet: Signer<'info>,
    pub system_program: Program<'info, System>,
}