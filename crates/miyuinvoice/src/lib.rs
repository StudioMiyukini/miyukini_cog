#![allow(missing_docs)]
//! # MiyuInvoice — toolkit.invoice.standalone
//!
//! Kit d'outils facturation métier (devis, factures, électronique, relances, lien paiement, clients).
//! Persistance = KindMother (WriteIntent) ; décisions (relance, devis→facture) = StrongFather.
//! Alignement MIP : domaine `invoice`, layer tool/toolkit.

pub mod admin_cell;
pub mod context;
pub mod customer;
pub mod electronic;
pub mod errors;
pub mod invoice;
pub mod payment_link;
pub mod quote;
pub mod reminder;
pub mod store;

pub use admin_cell::{
    miyuinvoice_admin_cell, MiyuinvoiceAdminCell, MiyuinvoiceIdentification, MiyuinvoiceIntegrity,
    MiyuinvoiceTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use customer::{list as customer_list, register as customer_register, resolve as customer_resolve};
pub use electronic::submit as electronic_submit;
pub use errors::MiyuinvoiceError;
pub use invoice::{create as invoice_create, send as invoice_send};
pub use payment_link::generate as payment_link_generate;
pub use quote::{create as quote_create, to_invoice as quote_to_invoice, update as quote_update};
pub use reminder::send as reminder_send;

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> GovernedContext {
        GovernedContext::new("test-mandate".to_string(), 1)
    }

    #[test]
    fn customer_register_list_resolve() {
        let c = ctx();
        customer_register(&c, "cust-1", "{\"name\":\"Acme\"}").unwrap();
        customer_register(&c, "cust-2", "{\"name\":\"Beta\"}").unwrap();
        let list = customer_list(&c, None).unwrap();
        assert!(list.contains(&"cust-1".to_string()));
        assert!(list.contains(&"cust-2".to_string()));
        assert_eq!(customer_resolve(&c, "cust-1").unwrap(), "{\"name\":\"Acme\"}");
    }

    #[test]
    fn customer_resolve_not_found() {
        let c = ctx();
        assert!(customer_resolve(&c, "nonexistent").is_err());
    }

    #[test]
    fn invoice_create_send() {
        let c = ctx();
        let id = invoice_create(&c, "{\"total\":100}").unwrap();
        assert!(id.starts_with("inv:"));
        invoice_send(&c, &id, "email", None).unwrap();
    }

    #[test]
    fn quote_create_update_to_invoice() {
        let c = ctx();
        let qid = quote_create(&c, "{\"lines\":[]}").unwrap();
        assert!(qid.starts_with("quo:"));
        quote_update(&c, &qid, "{\"lines\":[{\"qty\":1}]}").unwrap();
        let inv_id = quote_to_invoice(&c, &qid).unwrap();
        assert!(inv_id.starts_with("inv:"));
    }

    #[test]
    fn quote_to_invoice_not_found() {
        let c = ctx();
        assert!(quote_to_invoice(&c, "quo-nonexistent").is_err());
    }

    #[test]
    fn electronic_submit_payment_link_reminder() {
        let c = ctx();
        let id = invoice_create(&c, "{}").unwrap();
        assert!(electronic_submit(&c, &id, None).unwrap().starts_with("submitted:"));
        assert!(payment_link_generate(&c, &id).unwrap().contains("pay.example"));
        reminder_send(&c, &id, None).unwrap();
    }

    #[test]
    fn no_mandate_refused() {
        let c = GovernedContext::new(String::new(), 0);
        assert!(customer_list(&c, None).is_err());
        assert!(invoice_create(&c, "{}").is_err());
        assert!(quote_create(&c, "{}").is_err());
    }
}
