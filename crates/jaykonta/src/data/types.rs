//! Types de persistance JayKonta.

/// Mouvement de grand livre ou budget.
#[derive(Debug, Clone)]
pub struct MovementRecord {
    /// Identifiant unique.
    pub id: String,
    /// Scope fonctionnel (`purse` | `account`).
    pub scope: String,
    /// Reference de contexte metier (edition, professionnel, etc.).
    pub context_ref: String,
    /// Categorie de mouvement.
    pub category: String,
    /// Montant signe.
    pub amount: f64,
    /// Devise.
    pub currency: String,
    /// Date ISO.
    pub movement_date: String,
    /// Source service.
    pub source_service: String,
}

/// Devis.
#[derive(Debug, Clone)]
pub struct QuoteRecord {
    /// Identifiant du devis.
    pub id: String,
    /// Scope fonctionnel.
    pub scope: String,
    /// Reference metier (edition/exhibitor/professional).
    pub context_ref: String,
    /// Contrepartie.
    pub counterparty_ref: String,
    /// Montant total.
    pub total: f64,
    /// Devise.
    pub currency: String,
    /// Statut.
    pub status: String,
    /// Date ISO.
    pub created_at: String,
}

/// Facture.
#[derive(Debug, Clone)]
pub struct InvoiceRecord {
    /// Identifiant facture.
    pub id: String,
    /// Scope fonctionnel.
    pub scope: String,
    /// Reference metier.
    pub context_ref: String,
    /// Contrepartie.
    pub counterparty_ref: String,
    /// Devis d'origine optionnel.
    pub quote_id: Option<String>,
    /// Montant total.
    pub total: f64,
    /// Montant regle.
    pub paid_amount: f64,
    /// Devise.
    pub currency: String,
    /// Statut.
    pub status: String,
    /// Date emission ISO.
    pub issued_at: String,
    /// Date echeance ISO optionnelle.
    pub due_at: Option<String>,
}

/// Paiement.
#[derive(Debug, Clone)]
pub struct PaymentRecord {
    /// Identifiant paiement.
    pub id: String,
    /// Facture cible.
    pub invoice_id: String,
    /// Montant regle.
    pub amount: f64,
    /// Devise.
    pub currency: String,
    /// Date paiement ISO.
    pub paid_at: String,
    /// Methode de paiement (tokenisee).
    pub method: String,
    /// Reference opaque.
    pub reference_opaque: String,
}

/// Rappel CK-INT-03.
#[derive(Debug, Clone)]
pub struct ReminderRecord {
    /// Identifiant rappel.
    pub id: String,
    /// Reference echeance.
    pub deadline_ref: String,
    /// Date d'echeance ISO.
    pub due_at: String,
    /// Label lisible.
    pub label: String,
    /// Contexte lie.
    pub context_ref: String,
    /// Service source.
    pub source_service: String,
}

/// Entree d'audit.
#[derive(Debug, Clone)]
pub struct AuditRecord {
    /// Identifiant audit.
    pub id: String,
    /// Contrat (CK-INT, CK-AUD, etc.).
    pub contract_id: String,
    /// Acteur.
    pub actor_ref: String,
    /// Operation.
    pub operation: String,
    /// Scope.
    pub scope: String,
    /// Objet technique.
    pub object_ref: String,
    /// Resultat.
    pub result: String,
    /// Payload serialise JSON.
    pub payload_json: String,
    /// Horodatage ISO.
    pub created_at: String,
}
