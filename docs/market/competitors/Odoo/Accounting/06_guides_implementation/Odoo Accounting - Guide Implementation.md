# Odoo Accounting — Guide d'Implémentation avec Bornage

## Contexte

Ce document fournit un **guide d'implémentation technique complet** pour développer l'équivalent Accounting dans Miyukini, avec **bornage fonctionnel**, **spécifications techniques**, et **plan de développement**.

**Références :**
- [Logique Métier](../00_logique_metier/Odoo%20Accounting%20-%20Logique%20Metier%20Complete.md)
- [Spécifications Opérateurs](./04_specifications_miyukini/Odoo%20Accounting%20-%20Specifications%20Operateurs%20Miyukini.md)
- [Guide Intégration COG](./05_integration_cog/Odoo%20Accounting%20-%20Guide%20Integration%20COG.md)

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture technique détaillée
- Spécifications des crates Rust
- Schémas de données
- API et contrats
- Plan de développement par phases
- Bornage fonctionnel (MVP → Complet)
- Critères d'acceptation

**Hors scope :**
- Implémentation complète du code (sera dans les crates)
- Tests unitaires détaillés (sera dans les tests)

---

## 1. Architecture Technique

### 1.1 Structure des Crates

```
crates/
├── miyukini-account-ledger/      # AccountLedger Opérateur
│   ├── src/
│   │   ├── lib.rs
│   │   ├── entry.rs              # Modèle Entry
│   │   ├── line.rs                # Modèle MoveLine
│   │   ├── validation.rs          # Validation écritures
│   │   └── admin_cell.rs          # Cellule admin
│   └── Cargo.toml
│
├── miyukini-account-journal/      # AccountJournal Opérateur
│   ├── src/
│   │   ├── lib.rs
│   │   ├── journal.rs             # Modèle Journal
│   │   ├── sequence.rs            # Gestion séquences
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-account-chart/        # AccountChart Opérateur
│   ├── src/
│   │   ├── lib.rs
│   │   ├── account.rs             # Modèle Account
│   │   ├── chart.rs               # Plan comptable
│   │   ├── import.rs              # Import plans standards
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-account-reconciliation/ # AccountReconciliation Opérateur
│   ├── src/
│   │   ├── lib.rs
│   │   ├── reconciliation.rs     # Modèle Reconciliation
│   │   ├── matching.rs            # Algorithme correspondance
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
└── miyukini-account-report/       # AccountReport Opérateur
    ├── src/
    │   ├── lib.rs
    │   ├── report.rs              # Génération rapports
    │   ├── formats.rs             # PDF, Excel
    │   └── admin_cell.rs
    └── Cargo.toml
```

### 1.2 Dépendances Principales

**Cores Miyukini :**
- `miyukini-kernel` : Kernel (Id, Logger, Clock)
- `miyukini-central` : Cores (StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy)
- `miyukini-admin` : Admin cell

**Kits existants :**
- `miyuinvoice` : Facturation
- `miyucptaledger` : Outils comptabilité (réconciliation, etc.)

**Externes :**
- `serde` : Sérialisation
- `chrono` : Dates
- `rust_decimal` : Montants décimaux
- `uuid` : Identifiants

---

## 2. Schémas de Données

### 2.1 Modèle Entry (Écriture Comptable)

```rust
// miyukini-account-ledger/src/entry.rs

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDate;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub id: EntryId,
    pub name: Option<String>,              // Numéro séquence (généré)
    pub date: NaiveDate,                   // Date comptable
    pub journal_id: JournalId,
    pub move_type: MoveType,
    pub state: EntryState,
    pub partner_id: Option<PartnerId>,
    pub ref_field: Option<String>,         // Référence
    pub line_ids: Vec<LineId>,             // Lignes d'écriture
    pub amount_total: Decimal,             // Montant total
    pub amount_untaxed: Decimal,           // Montant HT
    pub amount_tax: Decimal,              // Montant taxes
    pub currency_id: CurrencyId,
    pub company_id: CompanyId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MoveType {
    Entry,              // Écriture manuelle
    OutInvoice,         // Facture client
    OutRefund,          // Avoir client
    InInvoice,          // Facture fournisseur
    InRefund,           // Avoir fournisseur
    OutReceipt,         // Reçu de vente
    InReceipt,          // Reçu d'achat
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntryState {
    Draft,              // Brouillon
    Posted,             // Validé
    Cancel,             // Annulé
}

impl Entry {
    pub fn is_balanced(&self) -> bool {
        // Vérification équilibre comptable
        // SUM(debit) = SUM(credit)
        // À implémenter avec lecture des lignes depuis KindMother
        true
    }
    
    pub fn can_be_validated(&self) -> Result<(), ValidationError> {
        if !self.is_balanced() {
            return Err(ValidationError::Unbalanced);
        }
        // Autres vérifications...
        Ok(())
    }
}
```

### 2.2 Modèle MoveLine (Ligne d'Écriture)

```rust
// miyukini-account-ledger/src/line.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveLine {
    pub id: LineId,
    pub move_id: EntryId,
    pub account_id: AccountId,
    pub partner_id: Option<PartnerId>,
    pub name: String,                      // Libellé
    pub debit: Decimal,
    pub credit: Decimal,
    pub balance: Decimal,                  // debit - credit
    pub amount_currency: Option<Decimal>,  // Montant en devise étrangère
    pub currency_id: CurrencyId,
    pub date_maturity: Option<NaiveDate>,  // Date d'échéance
    pub reconciled: bool,
    pub display_type: DisplayType,
    pub sequence: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DisplayType {
    Product,           // Ligne produit/service
    Tax,              // Ligne taxe
    PaymentTerm,      // Ligne échéance
    Rounding,         // Arrondi
    LineSection,      // Section
    LineNote,         // Note
}
```

### 2.3 Modèle Account (Compte Comptable)

```rust
// miyukini-account-chart/src/account.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: AccountId,
    pub code: String,                      // Code compte (ex: "411000")
    pub name: String,                      // Libellé
    pub account_type: AccountType,
    pub reconcile: bool,                   // Permet réconciliation
    pub currency_id: Option<CurrencyId>,  // Devise (optionnel)
    pub company_ids: Vec<CompanyId>,       // Multi-company
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountType {
    AssetReceivable,      // Créances clients
    AssetCash,           // Banque et caisse
    AssetCurrent,        // Actifs courants
    AssetNonCurrent,     // Actifs non courants
    AssetPrepayments,    // Acomptes
    AssetFixed,          // Immobilisations
    LiabilityPayable,    // Dettes fournisseurs
    LiabilityCreditCard,  // Cartes de crédit
    LiabilityCurrent,    // Passifs courants
    LiabilityNonCurrent, // Passifs non courants
    Equity,              // Capitaux propres
    EquityUnaffected,    // Résultat exercice
    Income,              // Produits
    IncomeOther,         // Autres produits
    Expense,             // Charges
    ExpenseOther,        // Autres charges
    ExpenseDepreciation, // Amortissements
    ExpenseDirectCost,   // Coût des ventes
    OffBalance,          // Hors bilan
}
```

### 2.4 Modèle Journal (Journal Comptable)

```rust
// miyukini-account-journal/src/journal.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Journal {
    pub id: JournalId,
    pub name: String,
    pub code: String,                      // Code journal (ex: "VEN")
    pub journal_type: JournalType,
    pub default_account_id: AccountId,
    pub currency_id: Option<CurrencyId>,
    pub sequence_id: SequenceId,           // Séquence numérotation
    pub company_id: CompanyId,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JournalType {
    Sale,        // Journal ventes
    Purchase,    // Journal achats
    Bank,        // Journal bancaire
    Cash,        // Journal caisse
    General,     // Journal divers
}
```

---

## 3. API et Contrats

### 3.1 AccountLedger API

```rust
// miyukini-account-ledger/src/lib.rs

pub struct AccountLedgerOperator {
    // ...
}

impl AccountLedgerOperator {
    /// Création d'une écriture comptable
    pub async fn create_entry(
        &self,
        ctx: &OperatorContext,
        entry: EntryDraft,
    ) -> Result<EntryId, AccountError>;
    
    /// Validation d'une écriture
    pub async fn validate_entry(
        &self,
        ctx: &OperatorContext,
        entry_id: EntryId,
    ) -> Result<ValidatedEntry, AccountError>;
    
    /// Annulation d'une écriture
    pub async fn cancel_entry(
        &self,
        ctx: &OperatorContext,
        entry_id: EntryId,
    ) -> Result<(), AccountError>;
    
    /// Lecture d'une écriture
    pub async fn get_entry(
        &self,
        ctx: &OperatorContext,
        entry_id: EntryId,
    ) -> Result<Entry, AccountError>;
    
    /// Liste des écritures (avec filtres)
    pub async fn list_entries(
        &self,
        ctx: &OperatorContext,
        filters: EntryFilters,
    ) -> Result<Vec<Entry>, AccountError>;
    
    /// Ajout d'une ligne à une écriture
    pub async fn add_line(
        &self,
        ctx: &OperatorContext,
        entry_id: EntryId,
        line: LineDraft,
    ) -> Result<LineId, AccountError>;
    
    /// Suppression d'une ligne
    pub async fn remove_line(
        &self,
        ctx: &OperatorContext,
        entry_id: EntryId,
        line_id: LineId,
    ) -> Result<(), AccountError>;
}
```

### 3.2 AccountChart API

```rust
// miyukini-account-chart/src/lib.rs

pub struct AccountChartOperator {
    // ...
}

impl AccountChartOperator {
    /// Création d'un compte
    pub async fn create_account(
        &self,
        ctx: &OperatorContext,
        account: AccountDraft,
    ) -> Result<AccountId, AccountError>;
    
    /// Lecture d'un compte
    pub async fn get_account(
        &self,
        ctx: &OperatorContext,
        account_id: AccountId,
    ) -> Result<Account, AccountError>;
    
    /// Liste des comptes (avec filtres)
    pub async fn list_accounts(
        &self,
        ctx: &OperatorContext,
        filters: AccountFilters,
    ) -> Result<Vec<Account>, AccountError>;
    
    /// Import d'un plan comptable standard
    pub async fn import_chart(
        &self,
        ctx: &OperatorContext,
        chart_type: ChartType,  // PCG_France, etc.
    ) -> Result<Vec<AccountId>, AccountError>;
}
```

---

## 4. Plan de Développement par Phases

### 4.1 Phase 1 : MVP (Minimum Viable Product)

**Objectif :** Implémenter les fonctionnalités de base pour validation du concept.

**Fonctionnalités incluses :**
- [ ] Création d'écritures comptables manuelles
- [ ] Plan comptable basique (comptes principaux)
- [ ] Validation d'écritures (équilibre comptable)
- [ ] Consultation du grand livre
- [ ] Export CSV des écritures

**Fonctionnalités exclues :**
- ❌ Factures (utiliser MiyuInvoice existant)
- ❌ Réconciliations bancaires
- ❌ Multi-devises
- ❌ Taxes avancées
- ❌ Rapports avancés

**Durée estimée :** 4-6 semaines

**Critères d'acceptation :**
- Création d'écriture avec 2+ lignes
- Vérification équilibre comptable
- Validation d'écriture avec génération numéro
- Consultation liste écritures
- Export CSV fonctionnel

### 4.2 Phase 2 : Fonctionnalités Essentielles

**Objectif :** Ajouter les fonctionnalités essentielles pour usage professionnel.

**Fonctionnalités incluses :**
- [ ] Journaux comptables (ventes, achats, banque, caisse, divers)
- [ ] Séquencement automatique par journal
- [ ] Plan comptable complet (import PCG France)
- [ ] Gestion multi-devises (taux de change)
- [ ] Conditions de paiement et échéances
- [ ] Rapports de base (grand livre, balance)

**Fonctionnalités exclues :**
- ❌ Réconciliations bancaires avancées
- ❌ Comptabilité analytique
- ❌ Déclarations fiscales

**Durée estimée :** 6-8 semaines

**Critères d'acceptation :**
- Création de journaux avec séquences
- Import plan comptable PCG France
- Gestion multi-devises avec conversion
- Génération échéances selon conditions paiement
- Rapports grand livre et balance fonctionnels

### 4.3 Phase 3 : Fonctionnalités Avancées

**Objectif :** Ajouter les fonctionnalités avancées pour usage professionnel complet.

**Fonctionnalités incluses :**
- [ ] Réconciliations bancaires (import relevés, correspondance automatique)
- [ ] Taxes avancées (groupes, répartition)
- [ ] Comptabilité analytique (centres de coût)
- [ ] Rapports avancés (compte de résultat, bilan)
- [ ] Export PDF des rapports
- [ ] Hash d'inaltérabilité (conformité légale)

**Durée estimée :** 8-10 semaines

**Critères d'acceptation :**
- Import relevés bancaires (OFX, CSV)
- Correspondance automatique d'écritures
- Réconciliation manuelle fonctionnelle
- Génération compte de résultat et bilan
- Export PDF avec mise en page professionnelle
- Hash d'inaltérabilité SHA-256 fonctionnel

### 4.4 Phase 4 : Optimisations et Polish

**Objectif :** Optimiser les performances et améliorer l'expérience utilisateur.

**Fonctionnalités incluses :**
- [ ] Optimisation des requêtes (index, pagination)
- [ ] Génération asynchrone des rapports
- [ ] Cache des calculs fréquents
- [ ] Interface utilisateur améliorée
- [ ] Tests de charge et optimisation

**Durée estimée :** 4-6 semaines

---

## 5. Bornage Fonctionnel

### 5.1 MVP (Phase 1)

**Inclus :**
- ✅ Écritures comptables manuelles
- ✅ Plan comptable basique
- ✅ Validation avec équilibre
- ✅ Consultation grand livre
- ✅ Export CSV

**Exclus :**
- ❌ Factures (MiyuInvoice séparé)
- ❌ Journaux multiples
- ❌ Multi-devises
- ❌ Taxes
- ❌ Réconciliations
- ❌ Rapports avancés

### 5.2 Version Complète (Phase 3)

**Inclus :**
- ✅ Toutes fonctionnalités MVP
- ✅ Journaux comptables
- ✅ Plan comptable complet
- ✅ Multi-devises
- ✅ Conditions de paiement
- ✅ Réconciliations bancaires
- ✅ Taxes avancées
- ✅ Comptabilité analytique
- ✅ Rapports complets
- ✅ Hash d'inaltérabilité

**Exclus (hors scope) :**
- ❌ Déclarations fiscales automatiques (module séparé)
- ❌ Gestion de trésorerie avancée (module séparé)
- ❌ Multi-company avancé (gestion simplifiée uniquement)
- ❌ Intégration ERP complète (via API)

---

## 6. Spécifications Techniques Détaillées

### 6.1 Validation d'Écriture

**Algorithme :**

```rust
pub fn validate_entry(entry: &Entry) -> Result<(), ValidationError> {
    // 1. Vérification équilibre comptable
    let total_debit: Decimal = entry.line_ids.iter()
        .map(|line| line.debit)
        .sum();
    let total_credit: Decimal = entry.line_ids.iter()
        .map(|line| line.credit)
        .sum();
    
    if total_debit != total_credit {
        return Err(ValidationError::Unbalanced {
            debit: total_debit,
            credit: total_credit,
        });
    }
    
    // 2. Vérification au moins 2 lignes
    if entry.line_ids.len() < 2 {
        return Err(ValidationError::InsufficientLines);
    }
    
    // 3. Vérification comptes actifs
    for line in &entry.line_ids {
        let account = get_account(line.account_id)?;
        if !account.active {
            return Err(ValidationError::InactiveAccount(account.id));
        }
    }
    
    // 4. Vérification dates de verrouillage (WorrySentinel)
    // ...
    
    Ok(())
}
```

### 6.2 Génération de Séquence

**Algorithme :**

```rust
pub async fn generate_sequence_number(
    journal: &Journal,
    date: NaiveDate,
) -> Result<String, SequenceError> {
    // Format : {prefix}{year}{month}{seq}
    // Exemple : "VEN/2026/02/0001"
    
    let prefix = journal.code.clone();
    let year = date.year();
    let month = date.month();
    
    // Récupération dernière séquence du mois (Ever Buddy)
    let last_seq = ever_buddy::get_last_sequence(
        journal.sequence_id.clone(),
        year,
        month,
    ).await?;
    
    let next_seq = last_seq + 1;
    
    Ok(format!("{}/{}/{:02}/{:04}", prefix, year, month, next_seq))
}
```

### 6.3 Calcul d'Équilibre Comptable

**Algorithme :**

```rust
pub fn calculate_balance(lines: &[MoveLine]) -> BalanceResult {
    let total_debit: Decimal = lines.iter()
        .map(|line| line.debit)
        .sum();
    
    let total_credit: Decimal = lines.iter()
        .map(|line| line.credit)
        .sum();
    
    let balance = total_debit - total_credit;
    
    BalanceResult {
        total_debit,
        total_credit,
        balance,
        is_balanced: balance.is_zero(),
    }
}
```

---

## 7. Critères d'Acceptation par Phase

### 7.1 Phase 1 (MVP)

**AC-1.1 : Création d'écriture**
- GIVEN un utilisateur avec permissions
- WHEN il crée une écriture avec 2+ lignes équilibrées
- THEN l'écriture est créée en état Draft
- AND l'écriture est persistée dans KindMother
- AND l'équilibre comptable est vérifié

**AC-1.2 : Validation d'écriture**
- GIVEN une écriture en état Draft équilibrée
- WHEN l'utilisateur valide l'écriture
- THEN l'écriture passe en état Posted
- AND un numéro de séquence est généré
- AND l'écriture est persistée dans KindMother

**AC-1.3 : Consultation grand livre**
- GIVEN des écritures validées
- WHEN l'utilisateur consulte le grand livre
- THEN toutes les écritures sont affichées
- AND les montants sont corrects
- AND les filtres fonctionnent (date, journal, compte)

### 7.2 Phase 2 (Essentiel)

**AC-2.1 : Journaux avec séquences**
- GIVEN des journaux configurés
- WHEN une écriture est validée
- THEN le numéro suit la séquence du journal
- AND la séquence est unique par période

**AC-2.2 : Import plan comptable**
- GIVEN un plan comptable standard (PCG France)
- WHEN l'utilisateur importe le plan
- THEN tous les comptes sont créés
- AND la hiérarchie est respectée
- AND les comptes sont actifs

**AC-2.3 : Multi-devises**
- GIVEN une écriture en devise étrangère
- WHEN l'écriture est validée
- THEN le taux de change est figé
- AND les montants sont convertis en devise entreprise
- AND les montants en devise étrangère sont conservés

### 7.3 Phase 3 (Avancé)

**AC-3.1 : Réconciliations bancaires**
- GIVEN un relevé bancaire importé
- WHEN l'utilisateur lance la correspondance automatique
- THEN des suggestions de réconciliation sont proposées
- AND l'utilisateur peut valider manuellement
- AND les réconciliations sont persistées

**AC-3.2 : Rapports avancés**
- GIVEN des écritures validées
- WHEN l'utilisateur génère un compte de résultat
- THEN les montants sont corrects
- AND les comptes sont groupés par type
- AND l'export PDF est fonctionnel

---

## 8. Risques et Mitigation

### 8.1 Risques Identifiés

| Risque | Impact | Probabilité | Mitigation |
|--------|--------|-------------|------------|
| Complexité équilibre comptable | Élevé | Moyenne | Tests unitaires exhaustifs, validation à chaque étape |
| Performance sur grandes quantités | Moyen | Élevée | Pagination, index, cache |
| Gestion multi-devises | Moyen | Moyenne | Tests avec différents taux, gestion arrondis |
| Réconciliations automatiques imprécises | Faible | Élevée | Algorithme améliorable, validation manuelle toujours possible |

### 8.2 Stratégie de Tests

**Tests unitaires :**
- Calculs comptables (équilibre, balance)
- Génération séquences
- Validation écritures

**Tests d'intégration :**
- WriteIntent vers KindMother
- Décisions StrongFather
- Mandats de Permission

**Tests de performance :**
- Chargement grandes listes
- Génération rapports
- Requêtes complexes

---

## 9. Documentation Technique

### 9.1 Documentation Requise

- [ ] README par crate avec exemples d'usage
- [ ] Documentation API (rustdoc)
- [ ] Guide de développement (setup, build, test)
- [ ] Architecture décisionnelle (ADR) pour choix techniques
- [ ] Schémas de base de données (si applicable)

### 9.2 Standards de Code

- **Rust :** Suivre les conventions Rust (rustfmt, clippy)
- **Erreurs :** Utiliser `thiserror` pour les erreurs typées
- **Logging :** Utiliser le Logger du Kernel
- **Tests :** Couverture minimale 80% pour logique métier

---

## 10. Conclusion

Le **guide d'implémentation** fournit :

- **Architecture technique complète** : Structure des crates, schémas de données
- **API détaillées** : Contrats pour chaque Opérateur
- **Plan de développement** : 4 phases avec durées estimées
- **Bornage fonctionnel** : MVP → Version complète
- **Critères d'acceptation** : Tests par phase
- **Risques et mitigation** : Stratégie de tests

**Prochaines étapes :**
1. Valider l'architecture avec l'équipe
2. Démarrer Phase 1 (MVP)
3. Itérer selon feedback utilisateurs
4. Progresser vers Phase 2-4

---

**Document** : Odoo Accounting — Guide d'Implémentation avec Bornage  
**Version** : 1.0  
**Date** : 2026-02-01  
**Statut** : Guide complet — référence pour développement
