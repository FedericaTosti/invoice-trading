mod state;
mod processor;
mod errors;
mod context;

use crate::context::*;
use anchor_lang::prelude::*;

declare_id!("e8yzf4PFLh7o77GU9NivAbpTkBBYisz8mmuRiVSu4Se");

#[program]
pub mod invoice_trading {
    use super::*;

    pub fn create_borrower(
        ctx: Context<CreateBorrower>, 
        borrower_name: String, 
        borrower_address: String, 
        borrower_tax_id: String
    ) -> Result<()> {
        processor::create_borrower(ctx, borrower_name, borrower_address, borrower_tax_id)
    }

    pub fn create_investor(
        ctx: Context<CreateInvestor>, 
        investor_name: String, 
        investor_address: String
    ) -> Result<()> {
        processor::create_investor(ctx, investor_name, investor_address)
    }

    pub fn create_investment(
        ctx: Context<CreateInvestment>, 
        amount: u64
    ) -> Result<()> {
        processor::create_investment(ctx, amount)
    }

    pub fn fund_invoice(ctx: Context<FundInvoice>, amount: u64) -> Result<()> {
        processor::fund_invoice(ctx, amount)
    }

}
