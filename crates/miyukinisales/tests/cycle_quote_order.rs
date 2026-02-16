//! Test d'intégration : cycle devis → acceptation → commande.

use miyukinisales::{
    order_create_from_quote, order_confirm, quote_accept, quote_create, quote_send,
    MiyukiniSalesStore, OrderLine, OrderStatus, QuoteStatus,
};

#[test]
fn cycle_quote_then_order() {
    let store = MiyukiniSalesStore::new();

    let quote = quote_create(
        &store,
        "DEV-2026-001",
        "client-1",
        15000,
        "2026-12-31",
    )
    .unwrap();
    assert_eq!(quote.status, QuoteStatus::Draft);
    assert_eq!(quote.total_cents, 15000);

    quote_send(&store, &quote.id).unwrap();
    let q2 = store.quote_by_id(&quote.id).unwrap().unwrap();
    assert_eq!(q2.status, QuoteStatus::Sent);

    quote_accept(&store, &quote.id).unwrap();
    let q3 = store.quote_by_id(&quote.id).unwrap().unwrap();
    assert_eq!(q3.status, QuoteStatus::Accepted);

    let lines = vec![
        OrderLine {
            id: "L1".to_string(),
            label: "Article A".to_string(),
            quantity: 2,
            unit_price_cents: 5000,
            line_total_cents: 10000,
        },
        OrderLine {
            id: "L2".to_string(),
            label: "Article B".to_string(),
            quantity: 1,
            unit_price_cents: 5000,
            line_total_cents: 5000,
        },
    ];
    let order = order_create_from_quote(&store, &quote.id, "CMD-2026-001", lines).unwrap();
    assert_eq!(order.quote_id.as_deref(), Some(quote.id.as_str()));
    assert_eq!(order.status, OrderStatus::Pending);
    assert_eq!(order.total_cents, 15000);

    order_confirm(&store, &order.id).unwrap();
    let o2 = store.order_by_id(&order.id).unwrap().unwrap();
    assert_eq!(o2.status, OrderStatus::Confirmed);
}
