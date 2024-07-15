use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Borrower {
    pub owner: Pubkey, // Il proprietario dell'account del borrower (deve essere un utente autenticato)
    pub borrower_name: String, // Nome del borrower
    pub borrower_address: String, // Indirizzo del borrower
    pub borrower_tax_id: String, // Codice fiscale del borrower
    pub invoices: Vec<Pubkey>, // Lista di fatture create da questo borrower
}

impl Borrower {
    pub const LEN: usize = 8 // Discriminatore dell'account
        + 32 // Pubkey del proprietario
        + 4 + 64 // lunghezza massima del nome del borrower (String)
        + 4 + 128 // lunghezza massima dell'indirizzo del borrower (String)
        + 4 + 32 // lunghezza massima del codice fiscale del borrower (String)
        + 4 + 8 * 256; // spazio per una lista di 256 Pubkey, usato per la lista delle fatture
}

#[account]
#[derive(Default)]
pub struct Investor {
    pub owner: Pubkey,
    pub investor_name: String,
    pub investor_address: String,
    pub investments: Vec<Pubkey>,
}

impl Investor {
    pub const LEN: usize = 8 + 32 + 4 + 64 + 4 + 128 + 4 + 8 * 256;
}

#[account]
#[derive(Default)]
pub struct Investment {
    pub borrower: Pubkey,
    pub investor: Pubkey,
    pub amount: u64,
    pub repaid: bool,
}

impl Investment {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum InvoiceStatus {
    Pending,
    Funded,
    Paid,
    Refunded,
    Canceled,
    Reimbursed,
}

#[account]
pub struct Invoice {
    pub invoice_id: String,
    pub borrower: Pubkey,
    pub borrower_name: String,
    pub borrower_address: String,
    pub borrower_tax_id: String,
    pub client_name: String,
    pub client_address: String,
    pub client_tax_id: String,
    pub amount: u64,
    pub funded_amount: u64,
    pub currency: String,
    pub description: String,
    pub issue_date: i64,
    pub due_date: i64,
    pub token_mint: Pubkey,
    pub status: InvoiceStatus,
}

impl Invoice {
    pub const LEN: usize = 8 // Discriminatore dell'account
        + 32 // Pubkey del borrower
        + 4 + 64 // lunghezza massima dell'ID della fattura (String)
        + 4 + 64 // lunghezza massima del nome del borrower (String)
        + 4 + 128 // lunghezza massima dell'indirizzo del borrower (String)
        + 4 + 32 // lunghezza massima del codice fiscale del borrower (String)
        + 4 + 64 // lunghezza massima del nome del cliente (String)
        + 4 + 128 // lunghezza massima dell'indirizzo del cliente (String)
        + 4 + 32 // lunghezza massima del codice fiscale del cliente (String)
        + 8 // Importo
        + 4 + 3 // Valuta (String)
        + 4 + 256 // Descrizione della fattura (String)
        + 8 // Data di emissione
        + 8 // Data di scadenza
        + 1 // Stato della fattura (enum)
        + 32; // Pubkey dell'escrow
}