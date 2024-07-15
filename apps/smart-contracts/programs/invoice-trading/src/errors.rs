use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The invoice status is invalid for this operation.")]
    InvalidInvoiceStatus,

    #[msg("The refund amount exceeds the financed amount.")]
    ExceedsRefundAmount,

    #[msg("The refund request is invalid.")]
    InvalidRefundRequest,

    #[msg("The invoice is not fully funded.")]
    InvoiceNotFullyFunded,

    #[msg("The total amount to fund exceeds the invoice amount.")]
    ExceedsTotalAmount,

    #[msg("This is not a valid operation.")]
    InvalidOperation,
}