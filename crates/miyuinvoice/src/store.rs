//! Store en mémoire MiyuInvoice (clients, factures, devis). Persistance réelle = KindMother.

use std::collections::HashMap;
use std::sync::Mutex;

static CUSTOMERS: std::sync::OnceLock<Mutex<HashMap<String, String>>> = std::sync::OnceLock::new();
static INVOICES: std::sync::OnceLock<Mutex<HashMap<String, String>>> = std::sync::OnceLock::new();
static QUOTES: std::sync::OnceLock<Mutex<HashMap<String, String>>> = std::sync::OnceLock::new();

pub(crate) fn customers() -> &'static Mutex<HashMap<String, String>> {
    CUSTOMERS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub(crate) fn invoices() -> &'static Mutex<HashMap<String, String>> {
    INVOICES.get_or_init(|| Mutex::new(HashMap::new()))
}

pub(crate) fn quotes() -> &'static Mutex<HashMap<String, String>> {
    QUOTES.get_or_init(|| Mutex::new(HashMap::new()))
}
