use anchor_lang::prelude::*;
use crate::context::{CreateBorrower, CreateInvestor, CreateInvestment};

pub fn create_borrower(ctx: Context<CreateBorrower>, borrower_name: String, borrower_address: String, borrower_tax_id: String) -> Result<()> {
    let borrower = &mut ctx.accounts.borrower;
    borrower.owner = ctx.accounts.user.key();
    borrower.borrower_name = borrower_name;
    borrower.borrower_address = borrower_address;
    borrower.borrower_tax_id = borrower_tax_id;
    borrower.invoices = Vec::new();
    Ok(())
}


pub fn create_investor(
    ctx: Context<CreateInvestor>,
    investor_name: String,
    investor_address: String
) -> Result<()> {
    let investor = &mut ctx.accounts.investor;
    investor.owner = ctx.accounts.user.key();
    investor.investor_name = investor_name;
    investor.investor_address = investor_address;
    investor.investments = Vec::new();
    Ok(())
}

pub fn create_investment(
    ctx: Context<CreateInvestment>,
    amount: u64,
) -> Result<()> {
    let investment = &mut ctx.accounts.investment;
    investment.borrower = ctx.accounts.borrower.key();
    investment.investor = ctx.accounts.investor.key();
    investment.amount = amount;
    investment.repaid = false;
    ctx.accounts.borrower.invoices.push(ctx.accounts.investment.key());
    ctx.accounts.investor.investments.push(ctx.accounts.investment.key());
    Ok(())
}

pub fn fund_invoice(ctx: Context<FundInvoice>, amount: u64) -> Result<()> {
    let invoice = &mut ctx.accounts.invoice;
    let investor = &mut ctx.accounts.investor;

    if invoice.status == "Paid" {
        return Err(ErrorCode::ExceedsTotalAmount.into());
    }

    invoice.funded_amount += amount;
    if invoice.funded_amount >= invoice.amount {
        invoice.status = "Funded";
    }

    investor.investments.push(ctx.accounts.invoice.key());

    Ok(())
}