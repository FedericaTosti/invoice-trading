// src/tests/test_create_borrower.rs

use anchor_lang::prelude::*;
use anchor_client::solana_sdk::signature::Keypair;
use crate::program::InvoiceTrading; // Assicurati di importare il tuo programma
use crate::state::Borrower;

#[test]
fn test_create_borrower() {
    let program_id = Pubkey::new_unique(); // ID del programma, usa l'ID reale del tuo programma in un progetto reale
    let client = Client::new(program_id);

    let user = Keypair::new();
    let borrower = Keypair::new();

    let borrower_name = "Alice".to_string();
    let borrower_address = "123 Main St".to_string();
    let borrower_tax_id = "IT123456789".to_string();

    let ix = InvoiceTrading::create_borrower(
        &user,
        &borrower,
        borrower_name,
        borrower_address,
        borrower_tax_id,
    );

    let result = client.send(&ix);

    assert!(result.is_ok(), "La creazione del borrower è fallita");

    // Verifica che l'account sia stato creato correttamente
    let borrower_account = client.get_account::<Borrower>(&borrower.pubkey()).unwrap();
    assert_eq!(borrower_account.borrower_name, "Alice");
    assert_eq!(borrower_account.borrower_address, "123 Main St");
    assert_eq!(borrower_account.borrower_tax_id, "IT123456789");
    assert_eq!(borrower_account.invoices.len(), 0);
}
